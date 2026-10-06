//! Differential cases, part 2: the allocation registry and the key table.
//!
//! Same shape as part 1: real 32-bit objects on both sides, returns and
//! every effect compared, a deliberately wrong lift caught per method.
//! 32-bit target only.
//!
//! What each case pins beyond equality: `alloc_slot` pins the counter
//! before incrementing (including the wrap), the one-based live table
//! against the zero-based failure table, and the initialiser arguments;
//! `KeyTable::find` pins first-match-wins over the eight stride-spaced
//! records. One combination is out of domain: a failed allocation at a
//! wrapped counter writes far past the failure table, which no test can
//! plant, so the wrap case runs on the success path only.

#![allow(unsafe_code)]
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_input_frontend::input_slot::{
        KeyTable, REG_ENTRY_SIZE, RegBuild, RegEntry, SlotRegistry,
    };
    use lf_inputslot_diff::rewrites::*;
    use lf_inputslot_diff::rt;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{COUNT_VA, KEY_END_VA, KEY_TAB_VA, Rng, TAB_A_VA, TAB_B_VA, addr, lock};

    static RG_OK: Mutex<VecDeque<bool>> = Mutex::new(VecDeque::new());
    static RG_BLOCK: Mutex<u32> = Mutex::new(0);
    static RG_SIZES: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "cdecl" fn reg_alloc(size: u32) -> u32 {
        RG_SIZES.lock().unwrap().push(size);
        if RG_OK.lock().unwrap().pop_front().unwrap_or(false) {
            *RG_BLOCK.lock().unwrap()
        } else {
            0
        }
    }

    static RG_INIT_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static RG_INIT_BYTES: Mutex<[u8; REG_ENTRY_SIZE]> = Mutex::new([0; REG_ENTRY_SIZE]);
    /// Where the initialiser answers: the block, or a divergent address
    /// on one targeted case per run.
    static RG_INIT_ANSWER: Mutex<u32> = Mutex::new(0);
    extern "thiscall" fn reg_init(block: u32, arg: u32) -> u32 {
        RG_INIT_LOG.lock().unwrap().push((block, arg));
        let bytes = *RG_INIT_BYTES.lock().unwrap();
        let answer = *RG_INIT_ANSWER.lock().unwrap();
        let at = if answer == 0 { block } else { answer };
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), at as *mut u8, REG_ENTRY_SIZE);
        }
        at
    }

    fn clear_logs() {
        RG_OK.lock().unwrap().clear();
        RG_SIZES.lock().unwrap().clear();
        RG_INIT_LOG.lock().unwrap().clear();
    }

    #[derive(Debug)]
    struct Reg {
        ok: VecDeque<bool>,
        entries: VecDeque<RegEntry>,
        sizes: Vec<u32>,
        inits: Vec<(u32, u32)>,
        next_cookie: u32,
    }

    impl Reg {
        fn scripted(ok: bool, entry: RegEntry) -> Self {
            Self {
                ok: VecDeque::from([ok]),
                entries: VecDeque::from([entry]),
                sizes: Vec::new(),
                inits: Vec::new(),
                next_cookie: 1,
            }
        }
    }

    impl RegBuild for Reg {
        type Pending = u32;
        fn alloc(&mut self) -> Option<u32> {
            self.sizes.push(REG_ENTRY_SIZE as u32);
            if self.ok.pop_front().unwrap_or(false) {
                let cookie = self.next_cookie;
                self.next_cookie += 1;
                Some(cookie)
            } else {
                None
            }
        }
        fn init(&mut self, block: u32, arg: u32) -> RegEntry {
            self.inits.push((block, arg));
            self.entries.pop_front().unwrap()
        }
    }

    /// Reads one word through a raw pointer.
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }

    /// Reads bytes through a raw pointer.
    unsafe fn rd_bytes(a: u32, len: usize) -> Vec<u8> {
        unsafe { core::slice::from_raw_parts(a as *const u8, len).to_vec() }
    }

    /// The deliberately wrong lift: records successes as failures and
    /// failures as successes.
    fn wrong_alloc_slot(reg: &mut SlotRegistry, arg: u32, build: &mut Reg) -> u32 {
        let old = reg.count();
        match build.alloc() {
            // Swapped: a live entry lands in the failure list (as a
            // zero-length marker the comparison still sees), a failure
            // fabricates a blank live entry.
            Some(block) => {
                let _ = build.init(block, arg);
                let mut failed = reg.failed().to_vec();
                failed.push(old);
                *reg = SlotRegistry::with_state(old.wrapping_add(1), reg.live().to_vec(), failed);
            }
            None => {
                let mut live = reg.live().to_vec();
                live.push(RegEntry([0; REG_ENTRY_SIZE]));
                *reg = SlotRegistry::with_state(old.wrapping_add(1), live, reg.failed().to_vec());
            }
        }
        old
    }

    #[test]
    fn alloc_slot_matches() {
        let _guard = lock();
        clear_logs();
        rt::set_callee(1, reg_alloc as usize as u32);
        rt::set_callee(2, reg_init as usize as u32);
        let tab_a = Box::leak(Box::new([0u32; 64]));
        let tab_b = Box::leak(Box::new([0u32; 64]));
        rt::set_relocated(TAB_A_VA, addr(&tab_a[0]));
        rt::set_relocated(TAB_B_VA, addr(&tab_b[0]));
        let block = Box::leak(Box::new([0u8; REG_ENTRY_SIZE]));
        let block_addr = addr(&block[0]);
        let other = Box::leak(Box::new([0u8; REG_ENTRY_SIZE]));
        let other_addr = addr(&other[0]);
        let mut rng = Rng(0xA110);
        let mut cases = 0u32;
        let mut caught = 0u32;
        let counts = [0u32, 1, 5, u32::MAX];
        let args = [0u32, 1, u32::MAX];
        for case in 0..60 {
            let old = if case < counts.len() as u32 {
                counts[case as usize]
            } else {
                rng.below(8)
            };
            let arg = if case < args.len() as u32 {
                args[case as usize]
            } else {
                rng.u32()
            };
            // The wrap case runs on the success path only: a failure at a
            // wrapped counter writes far past the failure table.
            let ok = if old == u32::MAX { true } else { case % 3 != 0 };
            let mut bytes = [0u8; REG_ENTRY_SIZE];
            rng.bytes(&mut bytes);
            // One divergent-answer case: the initialiser answers a
            // different block than it was handed.
            let divergent = case == 7;
            for c in tab_a.iter_mut() {
                *c = 0;
            }
            for c in tab_b.iter_mut() {
                *c = 0;
            }
            unsafe { rt::global::<u32>(COUNT_VA).write(old) };
            RG_OK.lock().unwrap().push_back(ok);
            *RG_BLOCK.lock().unwrap() = block_addr;
            *RG_INIT_BYTES.lock().unwrap() = bytes;
            *RG_INIT_ANSWER.lock().unwrap() = if divergent { other_addr } else { 0 };
            let mut build = Reg::scripted(ok, RegEntry(bytes));
            let mut build_wrong = Reg::scripted(ok, RegEntry(bytes));
            let mut reg = SlotRegistry::with_state(old, Vec::new(), Vec::new());
            let mut reg_wrong = SlotRegistry::with_state(old, Vec::new(), Vec::new());
            let r_ret = unsafe { fn_009017E0::rw_009017e0(arg) };
            let l_ret = reg.alloc_slot(arg, &mut build);
            let w_ret = wrong_alloc_slot(&mut reg_wrong, arg, &mut build_wrong);
            let new = old.wrapping_add(1);
            assert_eq!(r_ret, old, "case {case}: return");
            assert_eq!(l_ret, old, "case {case}: lift return");
            assert_eq!(w_ret, old, "case {case}: wrong agrees on return");
            let count = unsafe { (rt::global::<u32>(COUNT_VA) as *const u32).read_unaligned() };
            assert_eq!(count, new, "case {case}: counter");
            assert_eq!(reg.count(), new, "case {case}: lift counter");
            assert_eq!(
                RG_SIZES.lock().unwrap().as_slice(),
                &[REG_ENTRY_SIZE as u32],
                "case {case}: alloc size"
            );
            assert_eq!(
                RG_SIZES.lock().unwrap().as_slice(),
                build.sizes.as_slice(),
                "case {case}: lift alloc sizes"
            );
            if ok {
                assert_eq!(
                    RG_INIT_LOG.lock().unwrap().as_slice(),
                    &[(block_addr, arg)],
                    "case {case}: init args"
                );
                assert_eq!(build.inits.len(), 1, "case {case}: lift init calls");
                assert_eq!(build.inits[0].1, arg, "case {case}: lift init arg");
                let stored =
                    unsafe { rd32(rt::relocated(TAB_A_VA).wrapping_add(new.wrapping_mul(4))) };
                let want = if divergent { other_addr } else { block_addr };
                assert_eq!(stored, want, "case {case}: live cell");
                let seen = unsafe { rd_bytes(stored, REG_ENTRY_SIZE) };
                assert_eq!(seen, bytes, "case {case}: live bytes");
                assert_eq!(reg.live(), &[RegEntry(bytes)], "case {case}: lift live");
                assert!(reg.failed().is_empty(), "case {case}: no failure");
                // No stray writes across either table.
                for (i, c) in tab_a.iter().enumerate() {
                    let seen_cell =
                        unsafe { rd32(rt::relocated(TAB_A_VA).wrapping_add((i as u32) * 4)) };
                    assert_eq!(seen_cell, *c, "case {case}: live table stable at {i}");
                }
            } else {
                assert!(
                    RG_INIT_LOG.lock().unwrap().is_empty(),
                    "case {case}: no init on failure"
                );
                assert!(build.inits.is_empty(), "case {case}: lift no init");
                let stored =
                    unsafe { rd32(rt::relocated(TAB_B_VA).wrapping_add(old.wrapping_mul(4))) };
                assert_eq!(stored, 0, "case {case}: failure cell null");
                assert!(reg.live().is_empty(), "case {case}: lift no live");
                assert_eq!(reg.failed(), &[old], "case {case}: lift failure");
            }
            // The wrong lift swaps the two lists: its live/failed never
            // match the rewrite's tables on any case.
            let w_live_ok = reg_wrong.live() == reg.live();
            let w_fail_ok = reg_wrong.failed() == reg.failed();
            if !w_live_ok || !w_fail_ok {
                caught += 1;
            }
            cases += 1;
            clear_logs();
        }
        assert_eq!(cases, 60);
        assert!(caught > 0, "wrong lift never caught");
    }

    #[test]
    fn key_find_matches() {
        let _guard = lock();
        clear_logs();
        let records = Box::leak(Box::new([0u8; 8 * 0x100]));
        let key_base = addr(&records[0]);
        rt::set_relocated(KEY_TAB_VA, key_base);
        rt::set_relocated(KEY_END_VA, key_base.wrapping_add(8 * 0x100));
        let obj = Box::leak(Box::new([0u8; 0xA0]));
        let obj_addr = addr(&obj[0]);
        let mut rng = Rng(0x4E87);
        let mut cases = 0u32;
        let mut caught = 0u32;
        for case in 0..60 {
            let mut ids = [0u32; 8];
            for id in ids.iter_mut() {
                *id = rng.u32();
            }
            // Targeted shapes: duplicates (first-match-wins), all equal,
            // key at each end, absent key.
            let key = match case % 6 {
                0 => {
                    ids[0] = 0xD9A1;
                    ids[3] = 0xD9A1;
                    0xD9A1
                }
                1 => {
                    ids = [0xE904; 8];
                    0xE904
                }
                2 => {
                    ids[7] = 0x8E9D;
                    0x8E9D
                }
                3 => 0xA8E97,
                _ => {
                    if case % 2 == 0 {
                        ids[(case as usize) % 8]
                    } else {
                        rng.u32() | 1
                    }
                }
            };
            for (i, id) in ids.iter().enumerate() {
                records[i * 0x100..i * 0x100 + 4].copy_from_slice(&id.to_le_bytes());
            }
            obj[0x60..0x64].copy_from_slice(&key.to_le_bytes());
            let table = KeyTable { ids };
            let r_ret = unsafe { fn_00928170::rw_00928170(obj_addr) };
            let l_ret = table.find(key);
            assert_eq!(r_ret, l_ret, "case {case}: return");
            // The wrong lift takes the last match instead of the first.
            let mut w_ret = u32::MAX;
            for (i, id) in ids.iter().enumerate() {
                if *id == key {
                    w_ret = i as u32;
                }
            }
            if w_ret != r_ret {
                caught += 1;
            }
            cases += 1;
        }
        assert_eq!(cases, 60);
        assert!(caught > 0, "wrong lift never caught");
    }
}
