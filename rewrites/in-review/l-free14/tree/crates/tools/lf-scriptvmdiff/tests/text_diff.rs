//! Differential cases, part 6: the text keys against their routine.
//!
//! Each case plants a state block, entry memory and the suffix/cookie
//! words, runs the rewrite and [`TextKeys::resolve_named_key`] on the
//! same key, and compares the probe, format, resolve and cookie calls in
//! order (snapshotting the formatted buffer inside the resolve stub),
//! the flag byte, and the returned entry. A deliberately wrong lift (the
//! fallback inverted: re-resolving on a hit, keeping a miss) must be
//! caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_scriptvmdiff::rewrites::*;
    use lf_scriptvmdiff::rt;
    use lf_script::script_vm::{
        FORMAT_LEN, TextEntry, TextKey, TextKeys, TextObj, EntryId,
    };

    #[path = "../support/mod.rs"]
    mod support;
    use support::{
        COOKIE_VA, FLAG_OFF, Rng, SUFFIX_VA, TEXT_OBJ_VA, addr, lock, snap,
    };

    /// State block size: the format flag is the last byte.
    const STATE_LEN: usize = FLAG_OFF + 1;

    static STATE_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static STATE_BLOCK: Mutex<u32> = Mutex::new(0);

    extern "cdecl" fn state_stub(arg: u32) -> u32 {
        STATE_LOG.lock().unwrap().push(arg);
        STATE_BLOCK.lock().unwrap().clone()
    }

    static FMT_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static FMT_SCRIPT: Mutex<VecDeque<[u8; FORMAT_LEN]>> = Mutex::new(VecDeque::new());

    extern "cdecl" fn fmt_stub(buf: u32, key: u32, len: u32) -> u32 {
        FMT_LOG.lock().unwrap().push((key, len));
        let bytes = FMT_SCRIPT.lock().unwrap().pop_front().unwrap();
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buf as *mut u8, FORMAT_LEN);
        }
        0
    }

    /// One recorded resolve call: the object, the raw key or the
    /// snapshotted formatted buffer.
    #[derive(Debug, PartialEq, Eq)]
    struct ResolveCall {
        obj: u32,
        key: TextKey,
    }

    static RESOLVE_LOG: Mutex<Vec<ResolveCall>> = Mutex::new(Vec::new());
    /// Scripted resolve answers: whether the argument is a buffer to
    /// snapshot, and the entry address to answer.
    static RESOLVE_SCRIPT: Mutex<VecDeque<(bool, u32)>> = Mutex::new(VecDeque::new());

    extern "thiscall" fn resolve_stub(this: u32, arg: u32) -> u32 {
        let (is_buf, entry) = RESOLVE_SCRIPT.lock().unwrap().pop_front().unwrap();
        let key = if is_buf {
            let snap = unsafe { snap(arg, FORMAT_LEN / 4) };
            let mut buf = [0u8; FORMAT_LEN];
            for (i, w) in snap.iter().enumerate() {
                buf[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
            }
            TextKey::Formatted(buf)
        } else {
            TextKey::Raw(arg)
        };
        RESOLVE_LOG.lock().unwrap().push(ResolveCall { obj: this, key });
        entry
    }

    static COOKIE_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());

    extern "thiscall" fn cookie_stub(this: u32) -> u32 {
        COOKIE_LOG.lock().unwrap().push(this);
        0
    }

    /// Formats a scripted buffer: random head, a short string from offset
    /// 12, its terminator, random tail. The terminator lands at 12..=20
    /// so the suffix splice stays inside the buffer.
    fn fmt_bytes(rng: &mut Rng, len: usize) -> [u8; FORMAT_LEN] {
        let mut buf = [0u8; FORMAT_LEN];
        rng.bytes(&mut buf[..12]);
        for i in 0..len {
            let mut b = 0;
            while b == 0 {
                b = rng.u32() as u8;
            }
            buf[12 + i] = b;
        }
        buf[12 + len] = 0;
        rng.bytes(&mut buf[12 + len + 1..]);
        buf
    }

    /// Maps a lift answer to the rewrite's word for comparison.
    fn entry_word(entry: Option<TextEntry>) -> u32 {
        entry.map(|e| e.id.get()).unwrap_or(0)
    }

    /// Runs the text-key routine over probe/flag/entry combinations.
    /// Returns (comparisons, caught).
    fn run_text(seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        rt::set_callee(1, state_stub as *const () as u32);
        rt::set_callee(2, fmt_stub as *const () as u32);
        rt::set_callee(3, resolve_stub as *const () as u32);
        rt::set_callee(4, cookie_stub as *const () as u32);
        // The entry object both sides resolve on.
        let obj_box = Box::new(0x5EEDu32);
        let obj_addr = addr(obj_box.as_ref());
        rt::set_relocated(TEXT_OBJ_VA, obj_addr);
        let obj = TextObj::new(obj_addr).expect("nonzero object");
        // Planted entries: two distinct non-empty, one empty.
        let e1 = Box::new([0x0041u16, 0]);
        let e2 = Box::new([0x0042u16, 0]);
        let e0 = Box::new([0u16, 0]);
        let addrs = [addr(e1.as_ref()), addr(e2.as_ref()), addr(e0.as_ref())];
        let (mut cases, mut caught) = (0, 0);
        // Probe flag bytes: clear, set, all-set; entries: hit, empty,
        // null; fallback: hit, empty; flag: null, live.
        let mut combos = Vec::new();
        for &flag_byte in &[0u8, 1, 0xFF] {
            for &first in &[0usize, 2, 3] {
                for &second in &[1usize, 2] {
                    for &live in &[false, true] {
                        combos.push((flag_byte, first, second, live));
                    }
                }
            }
        }
        for (flag_byte, first, second, live) in combos {
            let key = rng.u32();
            let suffix = rng.u32();
            let cookie = rng.u32();
            let fmt_len = (rng.u32() % 9) as usize;
            let fmt = fmt_bytes(&mut rng, fmt_len);
            let fmt_enabled = flag_byte != 0;
            // The first resolve answer and whether it hits.
            let (first_addr, hit) = if first == 3 {
                (0, false)
            } else {
                (addrs[first], first != 2)
            };
            let second_addr = addrs[second];
            let state = TextKeys::new(suffix, cookie);
            // Plant the state block with the scripted flag byte.
            let mut block = vec![0u8; STATE_LEN];
            block[FLAG_OFF] = flag_byte;
            *STATE_BLOCK.lock().unwrap() = addr(&block[0]);
            unsafe { rt::global::<u32>(SUFFIX_VA).write(suffix) };
            unsafe { rt::global::<u32>(COOKIE_VA).write(cookie) };
            STATE_LOG.lock().unwrap().clear();
            FMT_LOG.lock().unwrap().clear();
            FMT_SCRIPT.lock().unwrap().clear();
            RESOLVE_LOG.lock().unwrap().clear();
            RESOLVE_SCRIPT.lock().unwrap().clear();
            COOKIE_LOG.lock().unwrap().clear();
            if fmt_enabled {
                FMT_SCRIPT.lock().unwrap().push_back(fmt);
                RESOLVE_SCRIPT.lock().unwrap().push_back((true, first_addr));
                if !hit {
                    RESOLVE_SCRIPT.lock().unwrap().push_back((false, second_addr));
                }
            } else {
                RESOLVE_SCRIPT.lock().unwrap().push_back((false, first_addr));
            }
            // A live flag byte starts as garbage the rewrite overwrites.
            let mut flag_box = Box::new(0xAAu8);
            let flag_addr = if live { addr(flag_box.as_mut()) } else { 0 };
            let got = unsafe { fn_00B92370::rw_00b92370(key, flag_addr) };
            let want_ret = if fmt_enabled {
                if hit { first_addr } else { second_addr }
            } else {
                first_addr
            };
            assert_eq!(got, want_ret, "key={key:#x} fmt={fmt_enabled} hit={hit}");
            assert_eq!(*STATE_LOG.lock().unwrap(), [0]);
            assert_eq!(
                FMT_LOG.lock().unwrap().len(),
                usize::from(fmt_enabled),
                "formatter runs on the format path only"
            );
            if fmt_enabled {
                assert_eq!(*FMT_LOG.lock().unwrap(), [(key, FORMAT_LEN as u32)]);
            }
            assert_eq!(*COOKIE_LOG.lock().unwrap(), [cookie]);
            // The lift, scripted identically, must make the same calls.
            let mut lift_state_calls = 0;
            let mut lift_fmt_keys = Vec::new();
            let mut lift_resolves = Vec::new();
            let mut lift_cookies = Vec::new();
            let mut lift_entries = VecDeque::new();
            let to_entry = |a: u32| {
                if a == 0 {
                    None
                } else {
                    Some(TextEntry {
                        id: EntryId::new(a).expect("nonzero entry"),
                        nonempty: a != addrs[2],
                    })
                }
            };
            // The lift's entry queue, in expected-call order.
            lift_entries.push_back(to_entry(first_addr));
            if fmt_enabled && !hit {
                lift_entries.push_back(to_entry(second_addr));
            }
            let mut flag_bool = true;
            let flag_ref = live.then(|| &mut flag_bool);
            let lift_out = state.resolve_named_key(
                obj,
                &mut || {
                    lift_state_calls += 1;
                    fmt_enabled
                },
                &mut |k: u32| {
                    lift_fmt_keys.push(k);
                    fmt
                },
                &mut |o: TextObj, k: &TextKey| {
                    lift_resolves.push(ResolveCall {
                        obj: o.get(),
                        key: k.clone(),
                    });
                    lift_entries.pop_front().unwrap()
                },
                &mut |c: u32| lift_cookies.push(c),
                key,
                flag_ref,
            );
            assert_eq!(lift_state_calls, 1);
            assert_eq!(lift_fmt_keys, if fmt_enabled { vec![key] } else { vec![] });
            assert_eq!(lift_resolves, *RESOLVE_LOG.lock().unwrap());
            assert_eq!(lift_cookies, [cookie]);
            assert_eq!(entry_word(lift_out), got);
            if live {
                let seen = unsafe { (flag_addr as *const u8).read() };
                assert_eq!(seen, u8::from(flag_bool));
                let want_flag = fmt_enabled && hit;
                assert_eq!(flag_bool, want_flag);
            }
            // Wrong lift: the fallback inverted (re-resolving on a hit,
            // keeping a miss). Differs on a formatted hit (a spurious
            // second call) and on a formatted miss with a distinct
            // fallback answer.
            if fmt_enabled && (hit || second_addr != first_addr) {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box(&block);
            std::hint::black_box(&mut flag_box);
        }
        std::hint::black_box(&obj_box);
        std::hint::black_box(&e1);
        std::hint::black_box(&e2);
        std::hint::black_box(&e0);
        (cases, caught)
    }

    #[test]
    fn text_matches() {
        let _guard = lock();
        let (cases, caught) = run_text(0xF001);
        assert!(cases > 30, "too few comparisons ({cases})");
        assert!(caught > 0, "inverted fallback never caught ({cases} cases)");
    }

    // --- The dual-key dispatch ---

    static KEY_LOOKUP_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static KEY_LOOKUP_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());

    extern "cdecl" fn key_lookup_stub(key: u32, flag: u32) -> u32 {
        KEY_LOOKUP_LOG.lock().unwrap().push((key, flag));
        KEY_LOOKUP_SCRIPT.lock().unwrap().pop_front().unwrap()
    }

    static SETUP_PROBE_BLOCK: Mutex<u32> = Mutex::new(0);

    extern "cdecl" fn setup_probe_stub() -> u32 {
        SETUP_PROBE_BLOCK.lock().unwrap().clone()
    }

    static SETUP_LOG: Mutex<Vec<[u32; 14]>> = Mutex::new(Vec::new());

    extern "stdcall" fn setup_stub(
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
        a6: u32,
        a7: u32,
        a8: u32,
        a9: u32,
        a10: u32,
        a11: u32,
        a12: u32,
        a13: u32,
    ) -> u32 {
        SETUP_LOG.lock().unwrap().push([
            a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13,
        ]);
        0
    }

    static SEL_BLOCK: Mutex<u32> = Mutex::new(0);
    static SEL_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());

    extern "cdecl" fn sel_stub(arg: u32) -> u32 {
        SEL_LOG.lock().unwrap().push(arg);
        SEL_BLOCK.lock().unwrap().clone()
    }

    static DRAW_LOG: Mutex<Vec<[u32; 16]>> = Mutex::new(Vec::new());

    extern "cdecl" fn draw_keys_stub(
        a0: u32,
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
        a5: u32,
        a6: u32,
        a7: u32,
        a8: u32,
        a9: u32,
        a10: u32,
        a11: u32,
        a12: u32,
        a13: u32,
        a14: u32,
        a15: u32,
    ) -> u32 {
        DRAW_LOG.lock().unwrap().push([
            a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15,
        ]);
        0
    }

    static NOTIFY_A_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

    extern "thiscall" fn notify_a_stub(this: u32, key: u32) -> u32 {
        NOTIFY_A_LOG.lock().unwrap().push((this, key));
        0xDEAD_BEEF
    }

    static NOTIFY_B_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static NOTIFY_B_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());

    extern "thiscall" fn notify_b_stub(this: u32, key: u32) -> u32 {
        NOTIFY_B_LOG.lock().unwrap().push((this, key));
        NOTIFY_B_SCRIPT.lock().unwrap().pop_front().unwrap()
    }

    /// Runs the dual-key dispatch over gate combinations.
    /// Returns (comparisons, caught).
    fn run_dispatch(seed: u32) -> (u32, u32) {
        use support::{
            DISPATCH_OBJ_VA, DONE_VA, G1_VA, G2A_VA, G2B_VA, PROBE_OFF, SEL_OFF,
        };
        use lf_script::script_vm::{
            DispatchObj, LOOKUP_FLAG, SELECTOR_ARG, SETUP_NEG, SETUP_ONE, TextDispatch,
        };

        let mut rng = Rng(seed);
        rt::set_callee(1, key_lookup_stub as *const () as u32);
        rt::set_callee(2, setup_probe_stub as *const () as u32);
        rt::set_callee(3, setup_stub as *const () as u32);
        rt::set_callee(4, sel_stub as *const () as u32);
        rt::set_callee(5, draw_keys_stub as *const () as u32);
        rt::set_callee(6, notify_a_stub as *const () as u32);
        rt::set_callee(7, notify_b_stub as *const () as u32);
        let obj_box = Box::new(0xD15C10u32);
        let obj_addr = addr(obj_box.as_ref());
        rt::set_relocated(DISPATCH_OBJ_VA, obj_addr);
        let obj = DispatchObj::new(obj_addr).expect("nonzero object");
        let (mut cases, mut caught) = (0, 0);
        // Probe bytes: clear, set, all-set; gate pairs: unset, half-set,
        // agreeing, disagreeing; done bytes: clear, set, all-set.
        let mut combos = Vec::new();
        for &probe_byte in &[0u8, 1, 0xFF] {
            for &(g2a, g2b) in &[(0u32, 0u32), (0, 5), (5, 5), (5, 6)] {
                for &done_byte in &[0u8, 1, 0xFF] {
                    combos.push((probe_byte, g2a, g2b, done_byte));
                }
            }
        }
        for (probe_byte, g2a, g2b, done_byte) in combos {
            let (key0, key1, a2, a3) = (rng.u32(), rng.u32(), rng.u32(), rng.u32());
            let (r1, r2) = (rng.u32(), rng.u32());
            let g1 = rng.u32();
            let sel_byte = rng.u32() as u8;
            let out_word = rng.u32();
            let probe_nonzero = probe_byte != 0;
            let done = done_byte != 0;
            let setup_done = probe_nonzero || !(g2a != 0 && g2a != g2b);
            let draws = setup_done && done;
            let mut probe_block = vec![0u8; PROBE_OFF + 1];
            probe_block[PROBE_OFF] = probe_byte;
            *SETUP_PROBE_BLOCK.lock().unwrap() = addr(&probe_block[0]);
            let mut sel_block = vec![0u8; SEL_OFF + 1];
            sel_block[SEL_OFF] = sel_byte;
            *SEL_BLOCK.lock().unwrap() = addr(&sel_block[0]);
            unsafe { rt::global::<u32>(G1_VA).write(g1) };
            unsafe { rt::global::<u32>(G2A_VA).write(g2a) };
            unsafe { rt::global::<u32>(G2B_VA).write(g2b) };
            unsafe { rt::global::<u8>(DONE_VA).write(done_byte) };
            KEY_LOOKUP_LOG.lock().unwrap().clear();
            KEY_LOOKUP_SCRIPT.lock().unwrap().clear();
            KEY_LOOKUP_SCRIPT.lock().unwrap().push_back(r1);
            KEY_LOOKUP_SCRIPT.lock().unwrap().push_back(r2);
            SETUP_LOG.lock().unwrap().clear();
            SEL_LOG.lock().unwrap().clear();
            DRAW_LOG.lock().unwrap().clear();
            NOTIFY_A_LOG.lock().unwrap().clear();
            NOTIFY_B_LOG.lock().unwrap().clear();
            NOTIFY_B_SCRIPT.lock().unwrap().clear();
            NOTIFY_B_SCRIPT.lock().unwrap().push_back(out_word);
            let got = unsafe { fn_00B928D0::rw_00b928d0(key0, key1, a2, a3) };
            assert_eq!(got, out_word, "the second notify answer returns");
            assert_eq!(
                *KEY_LOOKUP_LOG.lock().unwrap(),
                [(key0, LOOKUP_FLAG), (key1, LOOKUP_FLAG)]
            );
            let want_setup = if setup_done {
                vec![[r1, 0, a2, 0, 0, r2, 0, 0, 0, 0, a3, 0, SETUP_ONE, SETUP_NEG]]
            } else {
                vec![]
            };
            assert_eq!(*SETUP_LOG.lock().unwrap(), want_setup);
            assert_eq!(
                SEL_LOG.lock().unwrap().len(),
                usize::from(draws),
                "the selector reads only when the draw fires"
            );
            if draws {
                assert_eq!(*SEL_LOG.lock().unwrap(), [SELECTOR_ARG]);
            }
            let neg = SETUP_NEG;
            let want_draw = if draws {
                vec![[
                    r1, g1, r2, g1, neg, neg, neg, neg, neg, neg, neg, neg, 0,
                    u32::from(sel_byte),
                    key0, key1,
                ]]
            } else {
                vec![]
            };
            assert_eq!(*DRAW_LOG.lock().unwrap(), want_draw);
            assert_eq!(unsafe { rt::global::<u8>(DONE_VA).read() }, 1);
            assert_eq!(*NOTIFY_A_LOG.lock().unwrap(), [(obj_addr, key0)]);
            assert_eq!(*NOTIFY_B_LOG.lock().unwrap(), [(obj_addr, key1)]);
            // The lift, scripted identically, must make the same calls.
            let mut state = TextDispatch::new(g1, [g2a, g2b], done);
            let mut lift_lookups = Vec::new();
            let mut lift_lookup_answers = VecDeque::from([r1, r2]);
            let mut lift_probes = 0;
            let mut lift_setups = Vec::new();
            let mut lift_sels = Vec::new();
            let mut lift_draws = Vec::new();
            let mut lift_a = Vec::new();
            let mut lift_b = Vec::new();
            let lift_out = state.dispatch(
                obj,
                &mut |key: u32, flag: u32| {
                    lift_lookups.push((key, flag));
                    lift_lookup_answers.pop_front().unwrap()
                },
                &mut || {
                    lift_probes += 1;
                    probe_nonzero
                },
                &mut |p0: u32,
                      p1: u32,
                      p2: u32,
                      p3: u32,
                      p4: u32,
                      p5: u32,
                      p6: u32,
                      p7: u32,
                      p8: u32,
                      p9: u32,
                      p10: u32,
                      p11: u32,
                      p12: u32,
                      p13: u32| {
                    lift_setups.push([
                        p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13,
                    ]);
                },
                &mut |arg: u32| {
                    lift_sels.push(arg);
                    sel_byte
                },
                &mut |p0: u32,
                      p1: u32,
                      p2: u32,
                      p3: u32,
                      p4: u32,
                      p5: u32,
                      p6: u32,
                      p7: u32,
                      p8: u32,
                      p9: u32,
                      p10: u32,
                      p11: u32,
                      p12: u32,
                      p13: u32,
                      p14: u32,
                      p15: u32| {
                    lift_draws.push([
                        p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14,
                        p15,
                    ]);
                },
                &mut |o: DispatchObj, key: u32| lift_a.push((o.get(), key)),
                &mut |o: DispatchObj, key: u32| {
                    lift_b.push((o.get(), key));
                    out_word
                },
                key0,
                key1,
                a2,
                a3,
            );
            assert_eq!(lift_lookups, *KEY_LOOKUP_LOG.lock().unwrap());
            assert_eq!(lift_probes, 1);
            assert_eq!(lift_setups, *SETUP_LOG.lock().unwrap());
            assert_eq!(lift_sels, *SEL_LOG.lock().unwrap());
            assert_eq!(lift_draws, *DRAW_LOG.lock().unwrap());
            assert_eq!(lift_a, *NOTIFY_A_LOG.lock().unwrap());
            assert_eq!(lift_b, *NOTIFY_B_LOG.lock().unwrap());
            assert_eq!(lift_out, got);
            assert!(state.done(), "the done flag rises on every path");
            // Wrong lift: the probe gate inverted (the pair check runs
            // when the probe byte is nonzero). Differs wherever the two
            // gates disagree.
            let wrong_setup = if probe_nonzero {
                !(g2a != 0 && g2a != g2b)
            } else {
                true
            };
            if wrong_setup != setup_done {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box(&probe_block);
            std::hint::black_box(&sel_block);
        }
        std::hint::black_box(&obj_box);
        (cases, caught)
    }

    #[test]
    fn dispatch_matches() {
        let _guard = lock();
        let (cases, caught) = run_dispatch(0xF002);
        assert!(cases > 30, "too few comparisons ({cases})");
        assert!(caught > 0, "inverted probe gate never caught ({cases} cases)");
    }
}
