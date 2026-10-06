//! Differential cases, part 5: voice activation and the bound check.
//!
//! Each case plants the voice object (flags, bitset, slots with shared
//! vtable), runs the rewrite and the lift with the same scripted world,
//! and compares the return, every written byte and word, and every
//! numbered and virtual-hook call in order. Each method has a
//! deliberately wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_audio::audio_voice::activation::{
        ActivationWorld, BITSET_WORDS, BitSet, BoundSlot, LockHandle, SLOT_COUNT, SlotHandle,
        VoiceActivation, VoiceLocks,
    };
    use lf_audiovoicediff::rewrites::*;
    use lf_audiovoicediff::rt::{self, StubKind};
    use lf_core::Handle32;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{ACTIVE_VA, COUNTER_VA, FILTER_VA, GATE_VA, HANDLE_VA, Rng, addr, lock};

    /// Voice object size: flags at the end plus one spare byte.
    const OBJ_LEN: usize = 0x3232;
    /// Live/active flag offsets.
    const LIVE: usize = 0x3230;
    /// Active flag offset.
    const ACTIVE: usize = 0x3231;
    /// Bitset address offset.
    const BITSET: usize = 0x28a0;
    /// First slot offset; slots are 8 bytes, first word only is read.
    const SLOTS: usize = 0xfa8;

    /// Fresh view of a test image; rebuilt after every rewrite call so no
    /// pre-call borrow is read back (the compiler would forward it).
    unsafe fn image(base: u32, len: usize) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(base as *mut u8, len) }
    }

    /// The scripted world: records translated calls and scripts answers.
    #[derive(Debug, PartialEq, Eq)]
    enum WCall {
        Reset,
        Lock(u32),
        Unlock(u32),
        Free(BitSet),
        Probe(u32),
        Notify(u32, u32),
    }

    struct Fake {
        calls: Vec<WCall>,
        probes: Vec<bool>,
    }

    impl Fake {
        fn new(probes: Vec<bool>) -> Self {
            Self {
                calls: Vec::new(),
                probes,
            }
        }
    }

    impl ActivationWorld for Fake {
        fn reset(&mut self) {
            self.calls.push(WCall::Reset);
        }
        fn lock(&mut self, handle: Option<LockHandle>) {
            self.calls.push(WCall::Lock(Handle32::raw_or_zero(handle)));
        }
        fn unlock(&mut self, handle: Option<LockHandle>) {
            self.calls
                .push(WCall::Unlock(Handle32::raw_or_zero(handle)));
        }
        fn free_bits(&mut self, bits: BitSet) {
            self.calls.push(WCall::Free(bits));
        }
        fn probe(&mut self, slot: SlotHandle) -> bool {
            self.calls.push(WCall::Probe(slot.get()));
            if self.probes.is_empty() {
                false
            } else {
                self.probes.remove(0)
            }
        }
        fn notify(&mut self, slot: SlotHandle, arg: u32) {
            self.calls.push(WCall::Notify(slot.get(), arg));
        }
    }

    /// A planted voice: object, bitset, slot objects and shared vtable.
    struct Fixture {
        obj: Box<[u8]>,
        bits: Option<Box<[u8]>>,
        objs: Box<[u8]>,
        vt: Box<[u8]>,
    }

    impl Fixture {
        fn this(&self) -> u32 {
            addr(&self.obj[0])
        }
        fn bits_addr(&self) -> u32 {
            self.bits.as_ref().map_or(0, |b| addr(&b[0]))
        }
        fn vt_addr(&self) -> u32 {
            addr(&self.vt[0])
        }
        fn obj_addr(&self, k: usize) -> u32 {
            addr(&self.objs[0]).wrapping_add(k as u32 * 4)
        }
    }

    /// Plants a voice with the given flags, bitset words (or null) and
    /// live-slot set. Slot objects point at one shared vtable carrying
    /// the hook stubs; unset-bit slots may hold null objects.
    fn plant(
        rng: &mut Rng,
        live: u8,
        active: u8,
        words: Option<[u32; BITSET_WORDS]>,
        live_slots: &[bool; SLOT_COUNT],
    ) -> Fixture {
        let bits = words.map(|ws| {
            let mut v = vec![0u8; BITSET_WORDS * 4];
            for (i, w) in ws.iter().enumerate() {
                v[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
            }
            let b: Box<[u8]> = v.into_boxed_slice();
            let _ = rng.u32();
            b
        });
        let mut vt = vec![0u8; 0x18];
        rng.bytes(&mut vt);
        vt[0x0c..0x10].copy_from_slice(&rt::notify_stub_addr().to_le_bytes());
        vt[0x14..0x18].copy_from_slice(&rt::probe_stub_addr().to_le_bytes());
        let fx = Fixture {
            obj: vec![0u8; OBJ_LEN].into_boxed_slice(),
            bits,
            objs: vec![0u8; SLOT_COUNT * 4].into_boxed_slice(),
            vt: vt.into_boxed_slice(),
        };
        // Random fill first; every planted word below survives it.
        unsafe {
            rng.bytes(image(fx.this(), OBJ_LEN));
        }
        let vt_addr = fx.vt_addr();
        let this = fx.this();
        let bits_addr = fx.bits_addr();
        for (k, live) in live_slots.iter().enumerate() {
            let o = fx.obj_addr(k);
            unsafe {
                std::ptr::write_unaligned(o as *mut u32, vt_addr);
                std::ptr::write_unaligned(
                    this.wrapping_add(SLOTS as u32).wrapping_add(k as u32 * 8) as *mut u32,
                    if *live { o } else { 0 },
                );
            }
        }
        unsafe {
            let img = image(this, OBJ_LEN);
            img[LIVE] = live;
            img[ACTIVE] = active;
            img[BITSET..BITSET + 4].copy_from_slice(&bits_addr.to_le_bytes());
            // Second words of slots stay random (never read).
        }
        fx
    }

    /// Decodes the lifted voice from a planted fixture.
    fn decode(fx: &Fixture) -> VoiceActivation {
        let img = unsafe { image(fx.this(), OBJ_LEN) };
        let bits = fx.bits.as_ref().map(|b| {
            let bi = unsafe { image(addr(&b[0]), BITSET_WORDS * 4) };
            let mut ws = [0u32; BITSET_WORDS];
            for (i, w) in ws.iter_mut().enumerate() {
                *w = u32::from_le_bytes(bi[i * 4..i * 4 + 4].try_into().unwrap());
            }
            ws
        });
        let mut slots = [None; SLOT_COUNT];
        for (k, s) in slots.iter_mut().enumerate() {
            let o = SLOTS + k * 8;
            let w = u32::from_le_bytes(img[o..o + 4].try_into().unwrap());
            *s = Handle32::new(w);
        }
        VoiceActivation {
            live: img[LIVE] != 0,
            active: img[ACTIVE] != 0,
            bits,
            slots,
        }
    }

    /// Decodes the shared globals into the lifted locks.
    fn decode_locks(handle: u32, counter: u32, gate: u32, filter: u32) -> VoiceLocks {
        VoiceLocks {
            handle: Handle32::new(handle),
            counter,
            gate: gate as u8 != 0,
            filter: filter as u8 != 0,
        }
    }

    /// Deliberately wrong lifts, each caught below.
    mod wrong {
        use lf_audio::audio_voice::activation::{
            ActivationWorld, BITSET_WORDS, VoiceActivation, VoiceLocks,
        };

        /// The update sweep without the probe: every set bit notifies.
        pub fn update_no_probe(
            act: &mut VoiceActivation,
            arg: u32,
            shared: &mut VoiceLocks,
            world: &mut impl ActivationWorld,
        ) {
            if !act.live {
                return;
            }
            if act.active {
                if !shared.gate {
                    world.reset();
                    let h = shared.handle;
                    world.lock(h);
                    shared.counter = shared.counter.wrapping_sub(1);
                    world.unlock(h);
                    act.active = false;
                    return;
                }
            } else {
                if !shared.gate {
                    return;
                }
                let h = shared.handle;
                world.lock(h);
                shared.counter = shared.counter.wrapping_add(1);
                world.unlock(h);
                act.active = true;
            }
            let bits = act.bits.clone().unwrap();
            // Sweep over the true words, notifying every set bit:
            let mut mask = 2u32;
            for (k, slot) in act.slots.iter().enumerate() {
                let i = k as u32 + 1;
                if bits[(i >> 5) as usize] & mask != 0 {
                    world.notify(slot.unwrap(), arg);
                }
                mask = mask.rotate_left(1);
            }
            let _ = BITSET_WORDS;
        }

        /// Teardown freeing the bitset before the deactivation.
        pub fn teardown_free_first(
            act: &mut VoiceActivation,
            shared: &mut VoiceLocks,
            world: &mut impl ActivationWorld,
        ) {
            if !act.live {
                return;
            }
            act.active = false;
            world.free_bits(act.bits.take().unwrap_or([0; BITSET_WORDS]));
            // Deactivation dropped with the reorder: no reset/lock calls.
            let _ = shared;
            act.live = false;
        }

        /// The bound check ignoring the id entirely.
        pub fn active_flag_only(slot: &super::BoundSlot, _current: u32) -> bool {
            slot.flag != 0
        }
    }

    /// Slot numbers (1-based) whose bits are set in `words`.
    fn set_slots(words: &[u32; BITSET_WORDS]) -> Vec<u32> {
        let mut out = Vec::new();
        let mut mask = 2u32;
        for k in 0..SLOT_COUNT {
            let i = k as u32 + 1;
            if words[(i >> 5) as usize] & mask != 0 {
                out.push(i);
            }
            mask = mask.rotate_left(1);
        }
        out
    }

    #[test]
    fn update_matches() {
        let _guard = lock();
        let mut rng = Rng(0x9EF0);
        let mut caught = 0;
        for trial in 0..48u32 {
            rt::clear_hooks();
            // Prelude rotation covers all five paths.
            let (live, active, gate) = match trial % 5 {
                0 => (0u8, 0u8, 0u32),
                1 => (1, 1, 0),
                2 => (1, 0, 0),
                3 => (1, 1, 1),
                _ => (1, 0, 1),
            };
            let filter = trial % 2 == 0;
            // Bitset rotation: empty, sparse, dense.
            let mut words = [0u32; BITSET_WORDS];
            match trial % 3 {
                0 => {}
                1 => {
                    for _ in 0..1 + rng.below(8) {
                        let i = 1 + rng.below(SLOT_COUNT as u32);
                        words[(i >> 5) as usize] |= 1 << (i & 31);
                    }
                }
                _ => {
                    for w in words.iter_mut() {
                        *w = rng.u32();
                    }
                }
            }
            let arg = rng.u32();
            let handle_raw = if trial % 4 == 0 { 0 } else { rng.u32() | 1 };
            let counter = match trial % 4 {
                0 => 0,
                1 => u32::MAX,
                _ => rng.u32(),
            };
            // Live objects on set-bit slots; nulls only where unset.
            let set = set_slots(&words);
            let mut live_slots = [false; SLOT_COUNT];
            for i in set.iter() {
                live_slots[(i - 1) as usize] = true;
            }
            for s in live_slots.iter_mut() {
                if !*s && rng.below(2) == 0 {
                    *s = true;
                }
            }
            let fx = plant(&mut rng, live, active, Some(words), &live_slots);
            let this = fx.this();
            rt::set_global(HANDLE_VA, handle_raw);
            rt::set_global(COUNTER_VA, counter);
            rt::set_global(GATE_VA, gate);
            rt::set_global(FILTER_VA, u32::from(filter));
            // Probe answers: low-byte/full-word disagreements included.
            let mut probe_words = Vec::new();
            for (n, _) in set.iter().enumerate() {
                probe_words.push(match (trial + n as u32) % 5 {
                    0 => 0x100, // nonzero word, zero low byte: skip
                    1 => 0,
                    2 => 0x1FF, // low byte set
                    3 => 1,
                    _ => rng.u32(),
                });
            }
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![0]),
                (2, StubKind::Cdecl1, vec![0]),
                (3, StubKind::Cdecl1, vec![0]),
            ]);
            rt::set_virtual(&[("probe", probe_words.clone()), ("notify", vec![])]);
            let initial_obj = unsafe { image(this, OBJ_LEN) }.to_vec();
            let initial_bits = unsafe { image(fx.bits_addr(), BITSET_WORDS * 4) }.to_vec();
            let mut lift = decode(&fx);
            let mut shared = decode_locks(handle_raw, counter, gate, u32::from(filter));
            let ret = unsafe { fn_008A9EF0::rw_008A9EF0(this, arg) };
            assert_eq!(ret, 0, "trial {trial}: the rewrite answers nothing");
            let numbered = rt::take_calls();
            let virtuals = rt::take_virtual();
            let probe_bools: Vec<bool> = probe_words.iter().map(|w| *w as u8 != 0).collect();
            let mut fake = Fake::new(probe_bools.clone());
            lift.update(arg, &mut shared, &mut fake);
            // Numbered calls: the prelude's reset/lock/unlock only.
            let expect_numbered = match (live != 0, active != 0, gate != 0) {
                (false, _, _) | (true, false, false) => vec![],
                (true, true, false) => vec![
                    (1, vec![this]),
                    (2, vec![handle_raw]),
                    (3, vec![handle_raw]),
                ],
                (true, false, true) => vec![(2, vec![handle_raw]), (3, vec![handle_raw])],
                (true, true, true) => vec![],
            };
            assert_eq!(
                numbered, expect_numbered,
                "trial {trial}: numbered prelude calls"
            );
            // Virtual log: probe per set slot iff filtering, notify iff running.
            let mut expect_virt = Vec::new();
            let falls_through =
                live != 0 && (active != 0 || gate != 0) && !(active != 0 && gate == 0);
            if falls_through {
                for (n, i) in set.iter().enumerate() {
                    let obj = fx.obj_addr((i - 1) as usize);
                    if filter {
                        expect_virt.push(("probe".to_string(), vec![obj]));
                        if !probe_bools[n] {
                            continue;
                        }
                    }
                    expect_virt.push(("notify".to_string(), vec![obj, arg]));
                }
            }
            assert_eq!(virtuals, expect_virt, "trial {trial}: virtual hook log");
            // Lifted fake log translates 1:1 (reset drops the this arg).
            let mut expect_fake = Vec::new();
            match (live != 0, active != 0, gate != 0) {
                (false, _, _) | (true, false, false) => {}
                (true, true, false) => {
                    expect_fake.push(WCall::Reset);
                    expect_fake.push(WCall::Lock(handle_raw));
                    expect_fake.push(WCall::Unlock(handle_raw));
                }
                (true, false, true) => {
                    expect_fake.push(WCall::Lock(handle_raw));
                    expect_fake.push(WCall::Unlock(handle_raw));
                }
                (true, true, true) => {}
            }
            if falls_through {
                for (n, i) in set.iter().enumerate() {
                    let obj = fx.obj_addr((i - 1) as usize);
                    if filter {
                        expect_fake.push(WCall::Probe(obj));
                        if !probe_bools[n] {
                            continue;
                        }
                    }
                    expect_fake.push(WCall::Notify(obj, arg));
                }
            }
            assert_eq!(fake.calls, expect_fake, "trial {trial}: lifted world log");
            // Effects: flags, counter global, untouched regions.
            let after = unsafe { image(this, OBJ_LEN) }.to_vec();
            assert_eq!(
                &after[..LIVE],
                &initial_obj[..LIVE],
                "trial {trial}: head survives"
            );
            assert_eq!(
                after[LIVE], live,
                "trial {trial}: live flag survives update"
            );
            let expect_active = match (live != 0, active != 0, gate != 0) {
                (false, _, _) => active,
                (true, true, false) => 0,
                (true, false, false) => 0,
                (true, _, true) => 1,
            };
            assert_eq!(after[ACTIVE], expect_active, "trial {trial}: active flag");
            assert_eq!(lift.active, expect_active != 0);
            let expect_counter = match (live != 0, active != 0, gate != 0) {
                (true, true, false) => counter.wrapping_sub(1),
                (true, false, true) => counter.wrapping_add(1),
                _ => counter,
            };
            assert_eq!(
                rt::get_global(COUNTER_VA),
                expect_counter,
                "trial {trial}: counter"
            );
            assert_eq!(shared.counter, expect_counter);
            assert_eq!(
                rt::get_global(HANDLE_VA),
                handle_raw,
                "trial {trial}: handle survives"
            );
            let after_bits = unsafe { image(fx.bits_addr(), BITSET_WORDS * 4) }.to_vec();
            assert_eq!(
                after_bits, initial_bits,
                "trial {trial}: bitset image survives"
            );
            // Wrong version: no probe. Differs iff filtering with a set
            // slot whose probe low byte is zero.
            if filter && probe_bools.iter().any(|b| !b) && falls_through {
                let mut w = decode(&fx);
                let mut wshared = decode_locks(handle_raw, counter, gate, u32::from(filter));
                let mut wf = Fake::new(probe_bools.clone());
                wrong::update_no_probe(&mut w, arg, &mut wshared, &mut wf);
                if wf.calls != fake.calls {
                    caught += 1;
                }
            } else if !filter && falls_through && !set.is_empty() {
                // Without the filter both agree; count nothing here.
            }
        }
        assert!(caught > 0, "no-probe mutant was never caught");
    }

    #[test]
    fn teardown_matches() {
        let _guard = lock();
        let mut rng = Rng(0xA440);
        let mut caught = 0;
        for trial in 0..48u32 {
            rt::clear_hooks();
            let live = if trial % 3 == 0 { 0u8 } else { 1 };
            let active = if trial % 2 == 0 { 0u8 } else { 1 };
            let null_bits = trial % 4 == 0;
            let mut words = [0u32; BITSET_WORDS];
            for w in words.iter_mut() {
                *w = rng.u32();
            }
            let handle_raw = if trial % 5 == 0 { 0 } else { rng.u32() | 1 };
            let counter = match trial % 3 {
                0 => 0,
                1 => u32::MAX,
                _ => rng.u32(),
            };
            let live_slots = [true; SLOT_COUNT];
            let fx = plant(
                &mut rng,
                live,
                active,
                if null_bits { None } else { Some(words) },
                &live_slots,
            );
            let this = fx.this();
            let bits_addr = fx.bits_addr();
            rt::set_global(HANDLE_VA, handle_raw);
            rt::set_global(COUNTER_VA, counter);
            rt::set_script(&[
                (1, StubKind::Thiscall1, vec![0]),
                (2, StubKind::Cdecl1, vec![0]),
                (3, StubKind::Cdecl1, vec![0]),
                (4, StubKind::Cdecl1, vec![0]),
            ]);
            let initial_obj = unsafe { image(this, OBJ_LEN) }.to_vec();
            let mut lift = decode(&fx);
            let mut shared = decode_locks(handle_raw, counter, 0, 0);
            let ret = unsafe { fn_008AA440::rw_008AA440(this) };
            assert_eq!(ret, 0, "trial {trial}: the rewrite answers nothing");
            let numbered = rt::take_calls();
            let expect_numbered = if live == 0 {
                vec![]
            } else if active != 0 {
                vec![
                    (1, vec![this]),
                    (2, vec![handle_raw]),
                    (3, vec![handle_raw]),
                    (4, vec![bits_addr]),
                ]
            } else {
                vec![(4, vec![bits_addr])]
            };
            assert_eq!(
                numbered, expect_numbered,
                "trial {trial}: teardown call order"
            );
            let mut fake = Fake::new(vec![]);
            lift.teardown(&mut shared, &mut fake);
            let freed = if null_bits { [0; BITSET_WORDS] } else { words };
            let expect_fake = if live == 0 {
                vec![]
            } else if active != 0 {
                vec![
                    WCall::Reset,
                    WCall::Lock(handle_raw),
                    WCall::Unlock(handle_raw),
                    WCall::Free(freed),
                ]
            } else {
                vec![WCall::Free(freed)]
            };
            assert_eq!(fake.calls, expect_fake, "trial {trial}: lifted world log");
            let after = unsafe { image(this, OBJ_LEN) }.to_vec();
            // The head survives except the nulled bitset word.
            assert_eq!(
                &after[..BITSET],
                &initial_obj[..BITSET],
                "trial {trial}: low head survives"
            );
            assert_eq!(
                &after[BITSET + 4..LIVE],
                &initial_obj[BITSET + 4..LIVE],
                "trial {trial}: high head survives"
            );
            if live == 0 {
                assert_eq!(
                    after, initial_obj,
                    "trial {trial}: quiet teardown writes nothing"
                );
            } else {
                assert_eq!(after[LIVE], 0, "trial {trial}: live clears");
                assert_eq!(after[ACTIVE], 0, "trial {trial}: active clears");
                assert_eq!(
                    u32::from_le_bytes(after[BITSET..BITSET + 4].try_into().unwrap()),
                    0,
                    "trial {trial}: bitset word nulls"
                );
                assert!(!lift.live && !lift.active && lift.bits.is_none());
            }
            let expect_counter = if live != 0 && active != 0 {
                counter.wrapping_sub(1)
            } else {
                counter
            };
            assert_eq!(
                rt::get_global(COUNTER_VA),
                expect_counter,
                "trial {trial}: counter"
            );
            assert_eq!(shared.counter, expect_counter);
            // Wrong version: free-first reorder. Differs iff the
            // deactivation runs (live + active).
            if live != 0 && active != 0 {
                let mut w = decode(&fx);
                let mut wshared = decode_locks(handle_raw, counter, 0, 0);
                let mut wf = Fake::new(vec![]);
                wrong::teardown_free_first(&mut w, &mut wshared, &mut wf);
                if wf.calls != fake.calls {
                    caught += 1;
                }
            }
        }
        assert!(caught > 0, "free-first mutant was never caught");
    }

    #[test]
    fn bound_check_matches() {
        let _guard = lock();
        let mut rng = Rng(0x74F00);
        let mut caught = 0;
        for trial in 0..48u32 {
            let flag = match trial % 4 {
                0 => 0u8,
                1 => 1,
                2 => 0xFF,
                _ => rng.u32() as u8,
            };
            let current = rng.u32();
            let id = match trial % 4 {
                0 => 0u32,
                1 => current,
                2 => current.wrapping_add(1),
                _ => rng.u32(),
            };
            let mut initial = vec![0u8; 0x120];
            rng.bytes(&mut initial);
            initial[0x11C] = flag;
            initial[0x114..0x118].copy_from_slice(&id.to_le_bytes());
            let obj: Box<[u8]> = initial.clone().into_boxed_slice();
            let this = addr(&obj[0]);
            rt::set_global(ACTIVE_VA, current);
            let lift = BoundSlot { flag, id };
            let ret = unsafe { fn_00974F00::rw_00974f00(this) };
            let out = lift.is_active(current);
            assert_eq!(ret, u32::from(out), "trial {trial}: answer matches");
            let after = unsafe { image(this, 0x120) }.to_vec();
            assert_eq!(after, initial, "trial {trial}: the check writes nothing");
            assert_eq!(
                rt::get_global(ACTIVE_VA),
                current,
                "trial {trial}: id global survives"
            );
            if wrong::active_flag_only(&lift, current) != out {
                caught += 1;
            }
        }
        assert!(caught > 0, "flag-only mutant was never caught");
    }
}
