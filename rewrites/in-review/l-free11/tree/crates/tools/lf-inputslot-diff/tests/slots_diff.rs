//! Differential cases, part 1: the handle-table slot store.
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares returns and every effect
//! (written bytes, published cells, callee call logs). Each method has a
//! deliberately wrong lift that must be caught. 32-bit target only.
//!
//! What each case pins beyond equality: `find_free` pins the low-byte-only
//! flag, the scan bound (no wild reads past a full table) and the
//! install-on-success/null-on-failure cells; `notify_kind` pins the sink
//! selection (the default slot's own kind byte is never consulted);
//! `flag_byte`/`mode_word` pin the shared resolve plus each read width and
//! offset; `announce` pins the exact flag bit and rebuilds the payload
//! address from the lifted answer on the inline path; `destroy` pins the
//! clear-then-drop-then-release order, the re-read of the slot, and the
//! low-byte result residue per path.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_inputslot_diff::rewrites::*;
    use lf_inputslot_diff::rt;
    use lf_input_frontend::input_slot::{
        Announce, AnnounceSink, DestroyOutcome, FormatPayload, NotifySinks, NotifyTarget, SlotBuild,
        SlotDrop, SlotLookup, SlotObject, SlotRelease, SlotStore, ThreadEntry, ANNOUNCE_BIT,
        ANNOUNCE_FLAG_OFF, BIG_SIZE, CLEAR_OFF, DEVICE_OFF, FLAG_OFF, HI_MASK, KIND_OFF, MODE_OFF,
        NOTIFY_ARG_OFF, PAYLOAD_LEN, SMALL_SIZE, TABLE_LEN,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        DEFAULT_VA, EMIT_SINK_VA, Rng, SINK_VAS, TABLE_VA, TLS_WORD_VA, addr, lock, put_u32,
    };

    // Recording stubs for the rewrite side. Each test plants the stubs
    // its rewrites call; the serial lock keeps the logs exact.

    static FF_SIZES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static FF_OK: Mutex<VecDeque<bool>> = Mutex::new(VecDeque::new());
    static FF_BLOCK: Mutex<u32> = Mutex::new(0);
    static FF_SMALL: Mutex<[u8; SMALL_SIZE]> = Mutex::new([0; SMALL_SIZE]);
    extern "cdecl" fn ff_alloc(size: u32) -> u32 {
        FF_SIZES.lock().unwrap().push(size);
        if FF_OK.lock().unwrap().pop_front().unwrap_or(false) {
            let block = *FF_BLOCK.lock().unwrap();
            if size == SMALL_SIZE as u32 {
                // The small path keeps allocator fill around the kind byte.
                let fill = *FF_SMALL.lock().unwrap();
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        fill.as_ptr(),
                        block as *mut u8,
                        SMALL_SIZE,
                    );
                }
            }
            block
        } else {
            0
        }
    }

    static FF_CTOR_THIS: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static FF_CTOR_BYTES: Mutex<[u8; BIG_SIZE]> = Mutex::new([0; BIG_SIZE]);
    extern "thiscall" fn ff_ctor(block: u32) -> u32 {
        FF_CTOR_THIS.lock().unwrap().push(block);
        let bytes = *FF_CTOR_BYTES.lock().unwrap();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), block as *mut u8, BIG_SIZE);
        }
        block
    }

    static N_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static N_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "thiscall" fn notify_stub(sink: u32, arg: u32) -> u32 {
        N_LOG.lock().unwrap().push((sink, arg));
        N_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    static FMT_LOG: Mutex<Vec<(u32, u32, u32)>> = Mutex::new(Vec::new());
    static FMT_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn fmt_stub(payload: u32, z1: u32, z2: u32) -> u32 {
        FMT_LOG.lock().unwrap().push((payload, z1, z2));
        FMT_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    static EMIT_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static EMIT_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "thiscall" fn emit_stub(sink: u32, text: u32) -> u32 {
        EMIT_LOG.lock().unwrap().push((sink, text));
        EMIT_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    static LK_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static LK_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn lookup_stub(id: u32) -> u32 {
        LK_LOG.lock().unwrap().push(id);
        ORDER.lock().unwrap().push('l');
        LK_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    /// What the drop stub does to the cell after logging.
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum DropMode {
        /// Leaves the cell alone.
        Keep,
        /// Clears the cell.
        Null,
        /// Installs another object address.
        Replace(u32),
    }

    static DR_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static DR_MODE: Mutex<DropMode> = Mutex::new(DropMode::Keep);
    extern "cdecl" fn drop_stub(idx: u32) -> u32 {
        DR_LOG.lock().unwrap().push(idx);
        ORDER.lock().unwrap().push('d');
        let tab = rt::relocated(TABLE_VA);
        unsafe {
            let cell = tab.wrapping_add(idx.wrapping_mul(4)) as *mut u32;
            match *DR_MODE.lock().unwrap() {
                DropMode::Keep => {}
                DropMode::Null => cell.write_unaligned(0),
                DropMode::Replace(a) => cell.write_unaligned(a),
            }
        }
        0
    }

    static REL_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static REL_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "cdecl" fn release_stub(obj: u32) -> u32 {
        REL_LOG.lock().unwrap().push(obj);
        ORDER.lock().unwrap().push('r');
        REL_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    /// Joint call order on the rewrite side (l/d/r).
    static ORDER: Mutex<Vec<char>> = Mutex::new(Vec::new());

    fn clear_logs() {
        FF_SIZES.lock().unwrap().clear();
        FF_OK.lock().unwrap().clear();
        FF_CTOR_THIS.lock().unwrap().clear();
        N_LOG.lock().unwrap().clear();
        N_SCRIPT.lock().unwrap().clear();
        FMT_LOG.lock().unwrap().clear();
        FMT_SCRIPT.lock().unwrap().clear();
        EMIT_LOG.lock().unwrap().clear();
        EMIT_SCRIPT.lock().unwrap().clear();
        LK_LOG.lock().unwrap().clear();
        LK_SCRIPT.lock().unwrap().clear();
        DR_LOG.lock().unwrap().clear();
        REL_LOG.lock().unwrap().clear();
        REL_SCRIPT.lock().unwrap().clear();
        ORDER.lock().unwrap().clear();
    }

    // Lift-side fakes mirroring the stubs.

    #[derive(Debug)]
    struct Build {
        sizes: Vec<u32>,
        ok: VecDeque<bool>,
        big: VecDeque<[u8; BIG_SIZE]>,
        small: VecDeque<[u8; SMALL_SIZE]>,
        constructed: Vec<u32>,
        next_cookie: u32,
    }

    impl Build {
        fn scripted(ok: bool, big: [u8; BIG_SIZE], small: [u8; SMALL_SIZE]) -> Self {
            Self {
                sizes: Vec::new(),
                ok: VecDeque::from([ok]),
                big: VecDeque::from([big]),
                small: VecDeque::from([small]),
                constructed: Vec::new(),
                next_cookie: 1,
            }
        }
    }

    impl SlotBuild for Build {
        type Pending = u32;
        fn alloc(&mut self, size: u32) -> Option<u32> {
            self.sizes.push(size);
            if self.ok.pop_front().unwrap_or(false) {
                let cookie = self.next_cookie;
                self.next_cookie += 1;
                Some(cookie)
            } else {
                None
            }
        }
        fn construct_big(&mut self, block: u32) -> [u8; BIG_SIZE] {
            self.constructed.push(block);
            self.big.pop_front().unwrap()
        }
        fn small_fill(&mut self, block: u32) -> [u8; SMALL_SIZE] {
            self.constructed.push(block);
            self.small.pop_front().unwrap()
        }
    }

    #[derive(Debug, Default)]
    struct Sinks {
        log: Vec<(NotifyTarget, u32)>,
        answers: VecDeque<u32>,
    }

    impl NotifySinks for Sinks {
        fn notify(&mut self, target: NotifyTarget, arg: u32) -> u32 {
            self.log.push((target, arg));
            self.answers.pop_front().unwrap_or(0)
        }
    }

    #[derive(Debug, Default)]
    struct Fmt {
        payloads: Vec<[u8; PAYLOAD_LEN]>,
        texts: VecDeque<u32>,
    }

    impl FormatPayload for Fmt {
        fn format(&mut self, payload: &[u8; PAYLOAD_LEN]) -> u32 {
            self.payloads.push(*payload);
            self.texts.pop_front().unwrap_or(0)
        }
    }

    #[derive(Debug, Default)]
    struct Emit {
        log: Vec<u32>,
        answers: VecDeque<u32>,
    }

    impl AnnounceSink for Emit {
        fn emit(&mut self, text: u32) -> u32 {
            self.log.push(text);
            self.answers.pop_front().unwrap_or(0)
        }
    }

    #[derive(Debug, Default)]
    struct Lookup {
        log: Vec<u32>,
        answers: VecDeque<u32>,
        order: Vec<char>,
    }

    impl SlotLookup for Lookup {
        fn lookup(&mut self, id: u32) -> u32 {
            self.log.push(id);
            self.order.push('l');
            self.answers.pop_front().unwrap_or(0)
        }
    }

    #[derive(Debug)]
    struct Drop {
        log: Vec<u32>,
        mode: DropMode,
        spare: SlotObject,
    }

    impl SlotDrop for Drop {
        fn drop_slot(&mut self, store: &mut SlotStore, idx: u32) {
            self.log.push(idx);
            match self.mode {
                DropMode::Keep => {}
                DropMode::Null => store.set_slot(idx, None),
                DropMode::Replace(_) => store.set_slot(idx, Some(self.spare.clone())),
            }
        }
    }

    #[derive(Debug, Default)]
    struct Release {
        log: Vec<Option<Vec<u8>>>,
        answers: VecDeque<u32>,
    }

    impl SlotRelease for Release {
        fn release(&mut self, slot: Option<&SlotObject>) -> u32 {
            self.log.push(slot.map(|s| s.as_bytes().to_vec()));
            self.answers.pop_front().unwrap_or(0)
        }
    }

    // Plant helpers. The rewrite-side boxes are leaked so their addresses
    // stay valid; each test leaks a bounded pool once (one table plus a
    // handful of records, tens of kilobytes at most) and rewrites cells
    // and bytes per case before the call. Every post-call read goes
    // through a raw pointer from the integer address, never through the
    // leaked reference, so the compiler cannot forward pre-call values.

    /// Reads one word through a raw pointer.
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }

    /// Reads bytes through a raw pointer.
    unsafe fn rd_bytes(a: u32, len: usize) -> Vec<u8> {
        unsafe { core::slice::from_raw_parts(a as *const u8, len).to_vec() }
    }

    /// Plants the global word at a VA.
    unsafe fn put_global(va: u32, v: u32) {
        unsafe { rt::global::<u32>(va).write(v) }
    }

    struct Table {
        base: u32,
        cells: &'static mut [u32; TABLE_LEN as usize],
    }

    fn plant_table() -> Table {
        let cells = Box::leak(Box::new([0u32; TABLE_LEN as usize]));
        let base = addr(&cells[0]);
        rt::set_relocated(TABLE_VA, base);
        Table { base, cells }
    }

    fn leak_big() -> (u32, &'static mut [u8; BIG_SIZE]) {
        let mem = Box::leak(Box::new([0u8; BIG_SIZE]));
        (addr(&mem[0]), mem)
    }

    fn leak_small() -> (u32, &'static mut [u8; SMALL_SIZE]) {
        let mem = Box::leak(Box::new([0u8; SMALL_SIZE]));
        (addr(&mem[0]), mem)
    }

    /// Writes bytes through a raw pointer (pre-call planting).
    unsafe fn wr_bytes(a: u32, bytes: &[u8]) {
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), a as *mut u8, bytes.len());
        }
    }

    /// The deliberately wrong lift: skips `start` (an off-by-one in the
    /// scan origin).
    fn wrong_find_free(store: &mut SlotStore, big: bool, start: u32, build: &mut Build) -> u32 {
        store.find_free(big, start.wrapping_add(1), build)
    }

    #[test]
    fn find_free_matches() {
        let _guard = lock();
        clear_logs();
        rt::set_callee(1, ff_alloc as usize as u32);
        rt::set_callee(2, ff_ctor as usize as u32);
        let mut table = plant_table();
        let (big_addr, _) = leak_big();
        let (small_addr, _) = leak_small();
        let mut big_pool = Vec::new();
        let mut small_pool = Vec::new();
        for _ in 0..3 {
            big_pool.push(leak_big().0);
        }
        for _ in 0..3 {
            small_pool.push(leak_small().0);
        }
        let mut rng = Rng(0xF1EE);
        let mut cases = 0u32;
        let mut caught = 0u32;
        // Edge starts: first slots, last slots, past the end, wild.
        let starts = [
            0u32,
            1,
            TABLE_LEN - 2,
            TABLE_LEN - 1,
            TABLE_LEN,
            TABLE_LEN + 1,
            u32::MAX,
        ];
        for case in 0..140 {
            let big = case % 2 == 0;
            let start = if (case as usize) < starts.len() {
                starts[case as usize]
            } else {
                rng.below(TABLE_LEN + 4)
            };
            // Occupancy: all empty, all full, empty from k on, random.
            let mut mirror = Vec::with_capacity(TABLE_LEN as usize);
            let k = rng.below(TABLE_LEN);
            for i in 0..TABLE_LEN {
                let full = match case % 4 {
                    0 => false,
                    1 => true,
                    2 => i < k,
                    _ => rng.below(2) == 0,
                };
                if !full {
                    table.cells[i as usize] = 0;
                    mirror.push(None);
                    continue;
                }
                if rng.below(2) == 0 {
                    let a = big_pool[(i as usize) % big_pool.len()];
                    let mut b = [0u8; BIG_SIZE];
                    rng.bytes(&mut b);
                    unsafe { wr_bytes(a, &b) };
                    table.cells[i as usize] = a;
                    mirror.push(Some(SlotObject::Big(b)));
                } else {
                    let a = small_pool[(i as usize) % small_pool.len()];
                    let mut b = [0u8; SMALL_SIZE];
                    rng.bytes(&mut b);
                    unsafe { wr_bytes(a, &b) };
                    table.cells[i as usize] = a;
                    mirror.push(Some(SlotObject::Small(b)));
                }
            }
            // Flag low byte selects the size; high garbage pins low-byte-only.
            let low = if big {
                if case % 3 == 0 { 0xFF } else { 1 }
            } else {
                0
            };
            let flag = (rng.u32() & 0xFFFF_FF00) | low;
            let alloc_ok = case % 3 != 0;
            let block = if big { big_addr } else { small_addr };
            let mut ctor_bytes = [0u8; BIG_SIZE];
            let mut small_bytes = [0u8; SMALL_SIZE];
            rng.bytes(&mut ctor_bytes);
            rng.bytes(&mut small_bytes);
            FF_OK.lock().unwrap().push_back(alloc_ok);
            *FF_BLOCK.lock().unwrap() = block;
            *FF_CTOR_BYTES.lock().unwrap() = ctor_bytes;
            *FF_SMALL.lock().unwrap() = small_bytes;
            let mut build = Build::scripted(alloc_ok, ctor_bytes, small_bytes);
            let mut build_wrong = Build::scripted(alloc_ok, ctor_bytes, small_bytes);
            let mut store = SlotStore::with_slots(mirror.clone(), 0);
            let mut store_wrong = SlotStore::with_slots(mirror, 0);
            let r_ret = unsafe { fn_00904250::rw_00904250(flag, start) };
            let l_ret = store.find_free(big, start, &mut build);
            assert_eq!(r_ret, l_ret, "case {case}: return");
            let installed = l_ret != u32::MAX;
            let size = if big { BIG_SIZE as u32 } else { SMALL_SIZE as u32 };
            assert_eq!(
                FF_SIZES.lock().unwrap().as_slice(),
                build.sizes.as_slice(),
                "case {case}: alloc sizes"
            );
            assert_eq!(
                build.sizes.as_slice(),
                if installed { &[size] } else { &[] },
                "case {case}: one alloc iff installed"
            );
            assert_eq!(
                FF_CTOR_THIS.lock().unwrap().as_slice(),
                if big && installed && alloc_ok {
                    &[block]
                } else {
                    &[]
                },
                "case {case}: ctor this-arg"
            );
            assert_eq!(
                build.constructed.len(),
                usize::from(installed && alloc_ok),
                "case {case}: lift construct calls"
            );
            if installed {
                let cell = unsafe { rd32(table.base.wrapping_add(l_ret.wrapping_mul(4))) };
                match (alloc_ok, &store.slots()[l_ret as usize]) {
                    (true, Some(obj)) => {
                        assert_eq!(cell, block, "case {case}: installed address");
                        let bytes = unsafe { rd_bytes(cell, obj.as_bytes().len()) };
                        assert_eq!(bytes, obj.as_bytes(), "case {case}: installed bytes");
                        if !big {
                            assert_eq!(bytes[KIND_OFF], 0, "case {case}: kind cleared");
                        }
                    }
                    (false, None) => assert_eq!(cell, 0, "case {case}: failed cell null"),
                    other => panic!("case {case}: impossible install state: {other:?}"),
                }
            }
            // No stray writes: every cell's null-ness matches the lift.
            for (i, slot) in store.slots().iter().enumerate() {
                let cell = unsafe { rd32(table.base.wrapping_add((i as u32).wrapping_mul(4))) };
                assert_eq!(cell == 0, slot.is_none(), "case {case}: cell {i}");
            }
            // The wrong lift must disagree somewhere observable.
            let w_ret = wrong_find_free(&mut store_wrong, big, start, &mut build_wrong);
            let r_installed = (r_ret != u32::MAX && alloc_ok).then(|| {
                let cell = unsafe { rd32(table.base.wrapping_add(r_ret.wrapping_mul(4))) };
                unsafe { rd_bytes(cell, size as usize) }
            });
            let w_installed = (w_ret != u32::MAX && alloc_ok).then(|| {
                store_wrong.slots()[w_ret as usize]
                    .as_ref()
                    .unwrap()
                    .as_bytes()
                    .to_vec()
            });
            if (w_ret, w_installed) != (r_ret, r_installed) {
                caught += 1;
            }
            cases += 1;
            clear_logs();
        }
        assert_eq!(cases, 140);
        assert!(caught > 0, "wrong lift never caught");
    }

    /// Maps a notified sink address back to its target.
    fn target_of(sink: u32, magics: &[u32; 3]) -> NotifyTarget {
        if sink == magics[0] {
            NotifyTarget::Sink1
        } else if sink == magics[1] {
            NotifyTarget::Sink2
        } else if sink == magics[2] {
            NotifyTarget::Sink3
        } else {
            panic!("notified unknown sink {sink:#x}");
        }
    }

    #[test]
    fn notify_kind_matches() {
        let _guard = lock();
        clear_logs();
        rt::set_callee(1, notify_stub as usize as u32);
        let mut table = plant_table();
        let (slot_addr, _) = leak_big();
        let (small_addr, _) = leak_small();
        let (def_addr, _) = leak_big();
        // Sink objects: distinct values the stub never dereferences.
        let magics = [0x5111_0001u32, 0x5111_0002, 0x5111_0003];
        for (i, va) in SINK_VAS.iter().enumerate() {
            unsafe { put_global(*va, magics[i]) };
        }
        let mut rng = Rng(0x9011);
        let mut cases = 0u32;
        let mut caught = 0u32;
        let devices = [0u32, 1, 2, 3, 4, 0x8000_0000, u32::MAX];
        for case in 0..120 {
            let idx = rng.below(TABLE_LEN);
            let default = rng.below(TABLE_LEN);
            // Slot shape: null, kind-set big, kind-zero big, small.
            let shape = case % 4;
            let mut slot_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut slot_bytes);
            slot_bytes[KIND_OFF] = match shape {
                1 => {
                    if case % 3 == 0 {
                        0xFF
                    } else {
                        1
                    }
                }
                _ => 0,
            };
            let device = devices[case as usize % devices.len()];
            put_u32(&mut slot_bytes, DEVICE_OFF, device);
            let arg = rng.u32();
            put_u32(&mut slot_bytes, NOTIFY_ARG_OFF, arg);
            let mut small_bytes = [0u8; SMALL_SIZE];
            rng.bytes(&mut small_bytes);
            small_bytes[KIND_OFF] = 0;
            let mut def_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut def_bytes);
            // The default slot's own kind byte is never consulted: leave
            // it zero half the time to pin that.
            def_bytes[KIND_OFF] = if case % 2 == 0 { 0 } else { 5 };
            let def_device = devices[(case as usize + 3) % devices.len()];
            put_u32(&mut def_bytes, DEVICE_OFF, def_device);
            for c in table.cells.iter_mut() {
                *c = 0;
            }
            let slot = match shape {
                0 => None,
                3 => {
                    unsafe { wr_bytes(small_addr, &small_bytes) };
                    table.cells[idx as usize] = small_addr;
                    Some(SlotObject::Small(small_bytes))
                }
                _ => {
                    unsafe { wr_bytes(slot_addr, &slot_bytes) };
                    table.cells[idx as usize] = slot_addr;
                    Some(SlotObject::Big(slot_bytes))
                }
            };
            unsafe { wr_bytes(def_addr, &def_bytes) };
            table.cells[default as usize] = def_addr;
            let answer = match case % 5 {
                0 => 0,
                1 => u32::MAX,
                _ => rng.u32(),
            };
            N_SCRIPT.lock().unwrap().push_back(answer);
            unsafe { put_global(DEFAULT_VA, default) };
            let mut mirror = vec![None; TABLE_LEN as usize];
            mirror[idx as usize] = slot;
            mirror[default as usize] = Some(SlotObject::Big(def_bytes));
            let store = SlotStore::with_slots(mirror, default);
            let mut sinks = Sinks::default();
            sinks.answers.push_back(answer);
            let r_ret = unsafe { fn_009042D0::rw_009042d0(idx) };
            let l_ret = store.notify_kind(idx, &mut sinks);
            assert_eq!(r_ret, l_ret, "case {case}: return");
            let r_log: Vec<(NotifyTarget, u32)> = N_LOG
                .lock()
                .unwrap()
                .iter()
                .map(|(s, a)| (target_of(*s, &magics), *a))
                .collect();
            assert_eq!(r_log, sinks.log, "case {case}: notify log");
            // The wrong lift swaps the first two sinks.
            let w_log: Vec<(NotifyTarget, u32)> = sinks
                .log
                .iter()
                .map(|(t, a)| {
                    let w = match t {
                        NotifyTarget::Sink1 => NotifyTarget::Sink2,
                        NotifyTarget::Sink2 => NotifyTarget::Sink1,
                        NotifyTarget::Sink3 => NotifyTarget::Sink3,
                    };
                    (w, *a)
                })
                .collect();
            if w_log != r_log {
                caught += 1;
            }
            cases += 1;
            clear_logs();
        }
        assert_eq!(cases, 120);
        assert!(caught > 0, "wrong lift never caught");
    }

    #[test]
    fn flag_byte_matches() {
        let _guard = lock();
        clear_logs();
        let mut table = plant_table();
        let (slot_addr, _) = leak_big();
        let (small_addr, _) = leak_small();
        let (def_addr, _) = leak_big();
        let mut rng = Rng(0xF1A6);
        let mut cases = 0u32;
        let mut caught = 0u32;
        for case in 0..80 {
            let idx = rng.below(TABLE_LEN);
            let default = rng.below(TABLE_LEN);
            // Direct half the time (nonzero kind pins `!= 0`, not `== 1`),
            // fallback half the time (kind-zero big or small).
            let direct = case % 2 == 0;
            let mut slot_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut slot_bytes);
            slot_bytes[KIND_OFF] = if direct {
                if case % 3 == 0 { 0xFF } else { 1 }
            } else {
                0
            };
            // The mode word's first byte differs from the flag byte so the
            // swapped-offset wrong lift is always observable.
            slot_bytes[FLAG_OFF] = rng.u32() as u8;
            slot_bytes[MODE_OFF] = !slot_bytes[FLAG_OFF];
            let mut small_bytes = [0u8; SMALL_SIZE];
            rng.bytes(&mut small_bytes);
            small_bytes[KIND_OFF] = 0;
            let mut def_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut def_bytes);
            def_bytes[KIND_OFF] = 7;
            def_bytes[FLAG_OFF] = rng.u32() as u8;
            def_bytes[MODE_OFF] = !def_bytes[FLAG_OFF];
            for c in table.cells.iter_mut() {
                *c = 0;
            }
            let slot = if direct {
                unsafe { wr_bytes(slot_addr, &slot_bytes) };
                table.cells[idx as usize] = slot_addr;
                Some(SlotObject::Big(slot_bytes))
            } else if case % 4 == 1 {
                unsafe { wr_bytes(slot_addr, &slot_bytes) };
                table.cells[idx as usize] = slot_addr;
                Some(SlotObject::Big(slot_bytes))
            } else {
                unsafe { wr_bytes(small_addr, &small_bytes) };
                table.cells[idx as usize] = small_addr;
                Some(SlotObject::Small(small_bytes))
            };
            unsafe { wr_bytes(def_addr, &def_bytes) };
            table.cells[default as usize] = def_addr;
            unsafe { put_global(DEFAULT_VA, default) };
            let mut mirror = vec![None; TABLE_LEN as usize];
            mirror[idx as usize] = slot;
            mirror[default as usize] = Some(SlotObject::Big(def_bytes));
            let store = SlotStore::with_slots(mirror, default);
            let r_ret = unsafe { fn_00904990::rw_00904990(idx) };
            let l_ret = store.flag_byte(idx);
            assert_eq!(r_ret, l_ret as u32, "case {case}: return");
            // The wrong lift reads the mode word's first byte instead.
            let chosen = if direct { &slot_bytes } else { &def_bytes };
            if u32::from(chosen[MODE_OFF]) != r_ret {
                caught += 1;
            }
            cases += 1;
        }
        assert_eq!(cases, 80);
        assert!(caught > 0, "wrong lift never caught");
    }

    #[test]
    fn mode_word_matches() {
        let _guard = lock();
        clear_logs();
        let mut table = plant_table();
        let (slot_addr, _) = leak_big();
        let (small_addr, _) = leak_small();
        let (def_addr, _) = leak_big();
        let mut rng = Rng(0x100D);
        let mut cases = 0u32;
        let mut caught = 0u32;
        for case in 0..80 {
            let idx = rng.below(TABLE_LEN);
            let default = rng.below(TABLE_LEN);
            let direct = case % 2 == 0;
            let mut slot_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut slot_bytes);
            slot_bytes[KIND_OFF] = if direct {
                if case % 3 == 0 { 0xFF } else { 1 }
            } else {
                0
            };
            // The flag word differs from the mode word so the
            // swapped-offset wrong lift is always observable.
            let mode = rng.u32();
            put_u32(&mut slot_bytes, MODE_OFF, mode);
            put_u32(&mut slot_bytes, FLAG_OFF, !mode);
            let mut small_bytes = [0u8; SMALL_SIZE];
            rng.bytes(&mut small_bytes);
            small_bytes[KIND_OFF] = 0;
            let mut def_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut def_bytes);
            def_bytes[KIND_OFF] = 7;
            let def_mode = rng.u32();
            put_u32(&mut def_bytes, MODE_OFF, def_mode);
            put_u32(&mut def_bytes, FLAG_OFF, !def_mode);
            for c in table.cells.iter_mut() {
                *c = 0;
            }
            let slot = if direct || case % 4 == 1 {
                unsafe { wr_bytes(slot_addr, &slot_bytes) };
                table.cells[idx as usize] = slot_addr;
                Some(SlotObject::Big(slot_bytes))
            } else {
                unsafe { wr_bytes(small_addr, &small_bytes) };
                table.cells[idx as usize] = small_addr;
                Some(SlotObject::Small(small_bytes))
            };
            unsafe { wr_bytes(def_addr, &def_bytes) };
            table.cells[default as usize] = def_addr;
            unsafe { put_global(DEFAULT_VA, default) };
            let mut mirror = vec![None; TABLE_LEN as usize];
            mirror[idx as usize] = slot;
            mirror[default as usize] = Some(SlotObject::Big(def_bytes));
            let store = SlotStore::with_slots(mirror, default);
            let r_ret = unsafe { fn_009049C0::rw_009049c0(idx) };
            let l_ret = store.mode_word(idx);
            assert_eq!(r_ret, l_ret, "case {case}: return");
            // The wrong lift reads the word at the flag offset instead.
            let chosen = if direct { &slot_bytes } else { &def_bytes };
            let wrong = u32::from_le_bytes(chosen[FLAG_OFF..FLAG_OFF + 4].try_into().unwrap());
            if wrong != r_ret {
                caught += 1;
            }
            cases += 1;
        }
        assert_eq!(cases, 80);
        assert!(caught > 0, "wrong lift never caught");
    }

    /// The deliberately wrong lift: tests the neighbouring flag bit.
    fn wrong_announce(
        store: &SlotStore,
        idx: u32,
        fmt: &mut Fmt,
        sink: &mut Emit,
    ) -> Announce {
        let slots = store.slots();
        let obj = slots[idx as usize].as_ref().unwrap();
        let obj = if obj.kind() != 0 {
            obj
        } else {
            slots[store.default_index() as usize].as_ref().unwrap()
        };
        if obj.byte_at(ANNOUNCE_FLAG_OFF) & 0x80 != 0 {
            let text = fmt.format(obj.payload());
            Announce::Emitted(sink.emit(text))
        } else {
            Announce::Inline
        }
    }

    #[test]
    fn announce_matches() {
        let _guard = lock();
        clear_logs();
        rt::set_callee(1, fmt_stub as usize as u32);
        rt::set_callee(2, emit_stub as usize as u32);
        let mut table = plant_table();
        let (slot_addr, _) = leak_big();
        let (small_addr, _) = leak_small();
        let (def_addr, _) = leak_big();
        // The sink object: a value the stub never dereferences.
        let sink_magic = 0xE111_7000u32;
        rt::set_relocated(EMIT_SINK_VA, sink_magic);
        let mut rng = Rng(0xA440);
        let mut cases = 0u32;
        let mut caught = 0u32;
        // Flag bytes pinning the exact bit: clear, the bit alone, the
        // neighbour alone, both, all set, all-but-the-bit, random.
        let flags = [0x00u8, 0x40, 0x80, 0xC0, 0xFF, 0xBF];
        for case in 0..120 {
            let idx = rng.below(TABLE_LEN);
            let default = rng.below(TABLE_LEN);
            let direct = case % 2 == 0;
            let mut slot_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut slot_bytes);
            slot_bytes[KIND_OFF] = if direct { 1 } else { 0 };
            let mut small_bytes = [0u8; SMALL_SIZE];
            rng.bytes(&mut small_bytes);
            small_bytes[KIND_OFF] = 0;
            let mut def_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut def_bytes);
            def_bytes[KIND_OFF] = 7;
            // The flag byte lives on the chosen record only.
            let flag = if case < flags.len() as u32 {
                flags[case as usize]
            } else if case % 7 == 6 {
                rng.u32() as u8
            } else {
                flags[(case as usize) % flags.len()]
            };
            if direct {
                slot_bytes[ANNOUNCE_FLAG_OFF] = flag;
            } else {
                def_bytes[ANNOUNCE_FLAG_OFF] = flag;
            }
            for c in table.cells.iter_mut() {
                *c = 0;
            }
            let slot = if direct {
                unsafe { wr_bytes(slot_addr, &slot_bytes) };
                table.cells[idx as usize] = slot_addr;
                Some(SlotObject::Big(slot_bytes))
            } else if case % 4 == 1 {
                unsafe { wr_bytes(slot_addr, &slot_bytes) };
                table.cells[idx as usize] = slot_addr;
                Some(SlotObject::Big(slot_bytes))
            } else {
                unsafe { wr_bytes(small_addr, &small_bytes) };
                table.cells[idx as usize] = small_addr;
                Some(SlotObject::Small(small_bytes))
            };
            unsafe { wr_bytes(def_addr, &def_bytes) };
            table.cells[default as usize] = def_addr;
            unsafe { put_global(DEFAULT_VA, default) };
            let text = rng.u32();
            let answer = if case % 5 == 0 { 0 } else { rng.u32() };
            FMT_SCRIPT.lock().unwrap().push_back(text);
            EMIT_SCRIPT.lock().unwrap().push_back(answer);
            let mut mirror = vec![None; TABLE_LEN as usize];
            mirror[idx as usize] = slot;
            mirror[default as usize] = Some(SlotObject::Big(def_bytes));
            let store = SlotStore::with_slots(mirror.clone(), default);
            let store_wrong = SlotStore::with_slots(mirror, default);
            let mut fmt = Fmt::default();
            fmt.texts.push_back(text);
            let mut emit = Emit::default();
            emit.answers.push_back(answer);
            let mut fmt_wrong = Fmt::default();
            fmt_wrong.texts.push_back(text);
            let mut emit_wrong = Emit::default();
            emit_wrong.answers.push_back(answer);
            let expected_base = if direct { slot_addr } else { def_addr };
            let r_ret = unsafe { fn_009049F0::rw_009049f0(idx) };
            let l_ret = store.announce(idx, &mut fmt, &mut emit);
            let w_ret = wrong_announce(&store_wrong, idx, &mut fmt_wrong, &mut emit_wrong);
            if flag & ANNOUNCE_BIT == 0 {
                assert_eq!(l_ret, Announce::Inline, "case {case}: lift path");
                // The translation is proven, not assumed: rebuild the
                // payload address from the lifted answer and compare it.
                assert_eq!(
                    r_ret,
                    expected_base.wrapping_add(0x60),
                    "case {case}: inline address"
                );
                assert!(FMT_LOG.lock().unwrap().is_empty(), "case {case}: no format");
                assert!(EMIT_LOG.lock().unwrap().is_empty(), "case {case}: no emit");
            } else {
                assert_eq!(l_ret, Announce::Emitted(answer), "case {case}: lift path");
                assert_eq!(r_ret, answer, "case {case}: emitted answer");
                assert_eq!(
                    FMT_LOG.lock().unwrap().as_slice(),
                    &[(expected_base.wrapping_add(0x60), 0, 0)],
                    "case {case}: format args"
                );
                assert_eq!(
                    fmt.payloads.as_slice(),
                    &[unsafe {
                        rd_bytes(expected_base.wrapping_add(0x60), PAYLOAD_LEN)
                            .try_into()
                            .unwrap()
                    }],
                    "case {case}: formatted payload"
                );
                assert_eq!(
                    EMIT_LOG.lock().unwrap().as_slice(),
                    &[(sink_magic, text)],
                    "case {case}: emit args"
                );
                assert_eq!(emit.log.as_slice(), &[text], "case {case}: lift emit");
            }
            if w_ret != l_ret {
                caught += 1;
            }
            cases += 1;
            clear_logs();
        }
        assert_eq!(cases, 120);
        assert!(caught > 0, "wrong lift never caught");
    }

    /// The deliberately wrong lift: skips clearing the object's word.
    #[allow(clippy::too_many_arguments)]
    fn wrong_destroy(
        store: &mut SlotStore,
        id: u32,
        by_handle: bool,
        thread: &ThreadEntry,
        lookup: &mut Lookup,
        drop: &mut Drop,
        release: &mut Release,
    ) -> DestroyOutcome {
        let idx = if by_handle { lookup.lookup(id) } else { id };
        if (idx as i32) < 0 {
            return DestroyOutcome::Invalid;
        }
        if store.slots()[idx as usize].is_none() {
            return DestroyOutcome::Empty;
        }
        // No clear here: the bug.
        drop.drop_slot(store, idx);
        if thread.owned {
            let answer = release.release(store.slots()[idx as usize].as_ref());
            store.set_slot(idx, None);
            DestroyOutcome::Released(answer & HI_MASK)
        } else {
            store.set_slot(idx, None);
            DestroyOutcome::Cleared
        }
    }

    #[test]
    fn destroy_matches() {
        let _guard = lock();
        clear_logs();
        rt::set_callee(1, lookup_stub as usize as u32);
        rt::set_callee(2, drop_stub as usize as u32);
        rt::set_callee(3, release_stub as usize as u32);
        let mut table = plant_table();
        let (slot_addr, _) = leak_big();
        let (small_addr, _) = leak_small();
        let (spare_addr, _) = leak_big();
        let entry = Box::leak(Box::new([0u8; 16]));
        let entry_addr = addr(&entry[0]);
        let mut rng = Rng(0xDE57);
        let mut cases = 0u32;
        let mut caught = 0u32;
        let negatives = [0xFFFF_FFFFu32, 0x8000_0000, 0xFFFF_0000];
        let owned_words = [1u32, 0xFF, u32::MAX];
        for case in 0..150 {
            let by_handle = case % 2 == 0;
            // Lookup answers stay in the lift's domain: valid indexes or
            // negatives (huge positives read wild memory in the original).
            let lookup_answer = match case % 5 {
                0 => rng.below(TABLE_LEN),
                1 => negatives[case as usize % negatives.len()],
                2 => rng.below(TABLE_LEN),
                3 => rng.u32() | 0x8000_0000,
                _ => rng.below(TABLE_LEN),
            };
            let id = if by_handle {
                rng.u32()
            } else if case % 7 == 6 {
                negatives[case as usize % negatives.len()]
            } else {
                rng.below(TABLE_LEN)
            };
            let idx = if by_handle { lookup_answer } else { id };
            // Slot shape: big, small, null.
            let shape = case % 3;
            let mut slot_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut slot_bytes);
            slot_bytes[KIND_OFF] = 1;
            // The cleared word starts nonzero so the clear is observable.
            put_u32(&mut slot_bytes, CLEAR_OFF, 0xBEEF_0000 | (case & 0xFFFF));
            let mut small_bytes = [0u8; SMALL_SIZE];
            rng.bytes(&mut small_bytes);
            small_bytes[KIND_OFF] = 0;
            put_u32(&mut small_bytes, CLEAR_OFF, 0x5EED_0000 | (case & 0xFFFF));
            let mut spare_bytes = [0u8; BIG_SIZE];
            rng.bytes(&mut spare_bytes);
            spare_bytes[KIND_OFF] = 9;
            let owned = case % 4 < 2;
            let owned_word = if owned {
                owned_words[case as usize % owned_words.len()]
            } else {
                0
            };
            put_u32(&mut entry[..], 8, owned_word);
            // TLS slot edges of the 64-entry mirror.
            let tls_slot = [0u32, 7, 63][case as usize % 3];
            unsafe { put_global(TLS_WORD_VA, tls_slot) };
            rt::set_tls(tls_slot as usize, entry_addr);
            for c in table.cells.iter_mut() {
                *c = 0;
            }
            let (obj_addr, slot) = match shape {
                0 => {
                    unsafe { wr_bytes(slot_addr, &slot_bytes) };
                    (slot_addr, Some(SlotObject::Big(slot_bytes)))
                }
                1 => {
                    unsafe { wr_bytes(small_addr, &small_bytes) };
                    (small_addr, Some(SlotObject::Small(small_bytes)))
                }
                _ => (0, None),
            };
            // Only a non-negative in-domain index names a cell; negatives
            // return before any table access.
            let in_domain = (idx as i32) >= 0 && idx < TABLE_LEN;
            if in_domain {
                table.cells[idx as usize] = obj_addr;
            }
            unsafe { wr_bytes(spare_addr, &spare_bytes) };
            let drop_mode = match case % 3 {
                0 => DropMode::Keep,
                1 => DropMode::Null,
                _ => DropMode::Replace(spare_addr),
            };
            *DR_MODE.lock().unwrap() = drop_mode;
            let release_answer = match case % 6 {
                0 => 0,
                1 => 1,
                2 => 0xFF,
                3 => 0x100,
                4 => u32::MAX,
                _ => rng.u32(),
            };
            LK_SCRIPT.lock().unwrap().push_back(lookup_answer);
            REL_SCRIPT.lock().unwrap().push_back(release_answer);
            let mut mirror = vec![None; TABLE_LEN as usize];
            if in_domain {
                mirror[idx as usize] = slot;
            }
            let mut store = SlotStore::with_slots(mirror.clone(), 0);
            let mut store_wrong = SlotStore::with_slots(mirror, 0);
            let thread = ThreadEntry { owned };
            let mut lookup = Lookup::default();
            lookup.answers.push_back(lookup_answer);
            let mut lookup_wrong = Lookup::default();
            lookup_wrong.answers.push_back(lookup_answer);
            let mk_drop = || Drop {
                log: Vec::new(),
                mode: drop_mode,
                spare: SlotObject::Big(spare_bytes),
            };
            let mut drop = mk_drop();
            let mut drop_wrong = mk_drop();
            let mut release = Release::default();
            release.answers.push_back(release_answer);
            let mut release_wrong = Release::default();
            release_wrong.answers.push_back(release_answer);
            // `by_handle` travels as its low byte only: high garbage pins it.
            let by_word = (rng.u32() & 0xFFFF_FF00) | u32::from(by_handle);
            let r_ret = unsafe { fn_009061E0::rw_009061e0(id, by_word) };
            let l_ret = store.destroy(
                id,
                by_handle,
                &thread,
                &mut lookup,
                &mut drop,
                &mut release,
            );
            let w_ret = wrong_destroy(
                &mut store_wrong,
                id,
                by_handle,
                &thread,
                &mut lookup_wrong,
                &mut drop_wrong,
                &mut release_wrong,
            );
            // Rebuild the full answer word per path and compare it.
            let expected = match l_ret {
                DestroyOutcome::Invalid => idx & HI_MASK,
                DestroyOutcome::Empty => 0,
                DestroyOutcome::Released(hi) => hi | 1,
                DestroyOutcome::Cleared => entry_addr & HI_MASK | 1,
            };
            assert_eq!(r_ret, expected, "case {case}: return");
            assert_eq!(w_ret, l_ret, "case {case}: wrong agrees on outcome");
            let ran = matches!(
                l_ret,
                DestroyOutcome::Released(_) | DestroyOutcome::Cleared
            );
            if in_domain {
                let cell = unsafe { rd32(table.base.wrapping_add(idx.wrapping_mul(4))) };
                if ran {
                    assert_eq!(cell, 0, "case {case}: cell cleared");
                    assert_eq!(store.slots()[idx as usize], None, "case {case}: lift cleared");
                } else {
                    assert_eq!(
                        cell, obj_addr,
                        "case {case}: early return keeps the cell"
                    );
                }
            }
            if ran {
                // The clear lands on the original object, before the drop.
                let cleared = unsafe { rd32(obj_addr.wrapping_add(CLEAR_OFF as u32)) };
                assert_eq!(cleared, 0, "case {case}: object word cleared");
            }
            // The wrong lift skipped the clear: when the drop kept the
            // cell and the path is owned, its released bytes still hold
            // the nonzero pre-state word where the rewrite shows zero.
            if ran && owned && drop_mode == DropMode::Keep {
                let w_bytes = release_wrong.log[0].as_ref().unwrap();
                let w_word = u32::from_le_bytes(
                    w_bytes[CLEAR_OFF..CLEAR_OFF + 4].try_into().unwrap(),
                );
                let r_word = unsafe { rd32(obj_addr.wrapping_add(CLEAR_OFF as u32)) };
                assert_eq!(r_word, 0, "case {case}: rewrite cleared");
                if w_word != r_word {
                    caught += 1;
                }
            }
            // Call logs: lookup iff by handle, drop iff ran, release iff
            // ran and owned; order l < d < r.
            assert_eq!(
                LK_LOG.lock().unwrap().as_slice(),
                lookup.log.as_slice(),
                "case {case}: lookup log"
            );
            assert_eq!(
                LK_LOG.lock().unwrap().as_slice(),
                if by_handle { &[id] } else { &[] },
                "case {case}: lookup iff by handle"
            );
            assert_eq!(
                DR_LOG.lock().unwrap().as_slice(),
                drop.log.as_slice(),
                "case {case}: drop log"
            );
            assert_eq!(
                DR_LOG.lock().unwrap().as_slice(),
                if ran { &[idx] } else { &[] },
                "case {case}: drop iff ran"
            );
            let r_rel = REL_LOG.lock().unwrap().clone();
            assert_eq!(
                r_rel.len(),
                usize::from(ran && owned),
                "case {case}: release iff ran and owned"
            );
            assert_eq!(
                release.log.len(),
                r_rel.len(),
                "case {case}: lift release count"
            );
            if ran && owned {
                match &release.log[0] {
                    None => assert_eq!(r_rel[0], 0, "case {case}: released null"),
                    Some(bytes) => {
                        assert_ne!(r_rel[0], 0, "case {case}: released address");
                        let seen = unsafe { rd_bytes(r_rel[0], bytes.len()) };
                        assert_eq!(seen, *bytes, "case {case}: released bytes");
                    }
                }
            }
            let mut l_order = lookup.order.clone();
            if ran {
                l_order.push('d');
            }
            if ran && owned {
                l_order.push('r');
            }
            assert_eq!(
                ORDER.lock().unwrap().as_slice(),
                l_order.as_slice(),
                "case {case}: call order"
            );
            cases += 1;
            clear_logs();
        }
        assert_eq!(cases, 150);
        assert!(caught > 0, "wrong lift never caught");
    }
}
