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

    /// One planted entry: its address and whether it reads non-empty.
    #[derive(Clone, Copy)]
    struct Planted {
        addr: u32,
        nonempty: bool,
    }

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
}
