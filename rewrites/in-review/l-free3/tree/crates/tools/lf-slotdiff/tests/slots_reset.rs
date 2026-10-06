//! Differential cases, part 3: resets and the control block.
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares returns and every effect
//! (written bytes, out words, callee call logs). Each method has a
//! deliberately wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::sync::Mutex;

    use lf_slotdiff::rewrites::*;
    use lf_slotdiff::rt;
    use lf_streaming::slots::ControlBlock;
    use lf_streaming::slots::FlagBank;
    use lf_streaming::slots::IdArray;
    use lf_streaming::slots::LaneTable;
    use lf_streaming::slots::ResetSlot;
    use lf_streaming::slots::LANES;
    use lf_streaming::slots::LANE_TABLE_LEN;
    use lf_streaming::slots::SLOT_LEN;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{addr, get_u32, lock, put_u32, Rng};

    /// Length word offset of the id array.
    const ID_LEN_OFF: usize = 0x80;
    /// Flag bank bases of the control block.
    const BANK_A: usize = 0xF0;
    const BANK_B: usize = 0xFE;
    /// Entry base and stride of the control block.
    const CTRL_ENTRY: usize = 0x1EC;
    const CTRL_STRIDE: usize = 8;

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_streaming::slots::ControlBlock;
        use lf_streaming::slots::FlagBank;
        use lf_streaming::slots::IdArray;
        use lf_streaming::slots::LaneTable;
        use lf_streaming::slots::ResetSlot;
        use lf_streaming::slots::LANES;

        /// Keeps the top four flag bits instead of three.
        pub fn reset_keep_f0(slot: &mut ResetSlot) {
            slot.bytes[0] &= 0xF0;
            slot.bytes[1] = 0;
            let mut off = 0x08;
            while off <= 0x84 {
                slot.bytes[off..off + 4].copy_from_slice(&0u32.to_le_bytes());
                off += 4;
            }
        }

        /// Clears the second row instead of filling it with all-ones.
        pub fn table_zero_row1(table: &mut LaneTable) {
            table.row0 = [0; LANES];
            table.row1 = [0; LANES];
            table.tail = 0;
            table.tag = 0;
        }

        /// Searches from one past the start.
        pub fn find_skip_first(array: &IdArray, want: u32, start: u32) -> Option<u32> {
            array.find_from(want, start.wrapping_add(1))
        }

        /// Matches without consulting the flag bank (always calls).
        pub fn match_no_flag(
            block: &ControlBlock,
            key: u32,
            idx: u32,
            diff: u32,
            calls: &mut u32,
        ) -> bool {
            let _ = (block, key, idx);
            *calls += 1;
            diff == 0
        }

        /// Resets with the two releases swapped.
        pub fn reset_swapped(
            block: &mut ControlBlock,
            idx: u32,
            notify: &mut Vec<(u32, u32)>,
            releases: &mut Vec<u32>,
        ) {
            notify.push((idx, 1));
            releases.push(idx.wrapping_add(0x10).wrapping_mul(8));
            releases.push(idx.wrapping_add(2).wrapping_mul(8));
            let at = idx as usize;
            block.flags_a[at] = 0;
            block.entries[at] = [0; 8];
        }

        /// Reads the other bank.
        pub fn other_bank(bank: FlagBank) -> FlagBank {
            match bank {
                FlagBank::A => FlagBank::B,
                FlagBank::B => FlagBank::A,
            }
        }
    }

    /// Reads back planted bytes through a raw pointer, so the compiler
    /// cannot forward the pre-call value.
    fn read_back(base: u32, len: usize) -> Vec<u8> {
        unsafe { std::slice::from_raw_parts(base as *const u8, len) }.to_vec()
    }

    #[test]
    fn slot_reset_matches() {
        let _guard = lock();
        let mut rng = Rng(0x5EED);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..40 {
            let mut bytes = [0u8; SLOT_LEN];
            rng.bytes(&mut bytes);
            if trial == 0 {
                bytes = [0xFF; SLOT_LEN];
            }
            let mut planted = vec![0u8; SLOT_LEN].into_boxed_slice();
            planted.copy_from_slice(&bytes);
            let slot = addr(&planted[0]);
            let got = unsafe { fn_00AB5C20::rw_00ab5c20(slot) };
            assert_eq!(got, slot, "trial {trial}");
            let mut lift = ResetSlot { bytes };
            lift.reset();
            let back = read_back(slot, SLOT_LEN);
            assert_eq!(back, lift.bytes, "trial {trial}");
            let mut bad = ResetSlot { bytes };
            wrong::reset_keep_f0(&mut bad);
            if bad.bytes != lift.bytes {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 40 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn table_reset_matches() {
        let _guard = lock();
        let mut rng = Rng(0x7AB1);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..40 {
            let mut bytes = [0u8; LANE_TABLE_LEN];
            rng.bytes(&mut bytes);
            if trial == 0 {
                bytes = [0xA5; LANE_TABLE_LEN];
            }
            let mut planted = vec![0u8; LANE_TABLE_LEN].into_boxed_slice();
            planted.copy_from_slice(&bytes);
            let tab = addr(&planted[0]);
            let got = unsafe { fn_00AB5D00::rw_00ab5d00(tab) };
            assert_eq!(got, tab, "trial {trial}");
            let mut lift = LaneTable::from_bytes(bytes);
            lift.reset();
            let back = read_back(tab, LANE_TABLE_LEN);
            assert_eq!(back, lift.to_bytes(), "trial {trial}");
            let mut bad = LaneTable::from_bytes(bytes);
            wrong::table_zero_row1(&mut bad);
            if bad.to_bytes() != lift.to_bytes() {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 40 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn find_index_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF1D3);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..12 {
            let n = (trial % 6) as usize;
            let mut ids = vec![];
            for _ in 0..n {
                ids.push(rng.below(8));
            }
            // Lengths: exact, short, zero, negative, past the end.
            let lens = [n as i32, n as i32 - 1, 0, -1, n as i32 + 2];
            for &len in &lens {
                // Past-the-end lengths leave the lift's domain.
                if len > n as i32 {
                    continue;
                }
                let array = IdArray {
                    ids: ids.clone(),
                    len,
                };
                let id_room = if n == 0 { 4 } else { n * 4 };
                let mut full = vec![0u8; ID_LEN_OFF + 4 + id_room].into_boxed_slice();
                rng.bytes(&mut full);
                for (i, id) in ids.iter().enumerate() {
                    put_u32(&mut full, i * 4, *id);
                }
                put_u32(&mut full, ID_LEN_OFF, len as u32);
                let this = addr(&full[0]);
                for start in 0..n as u32 + 2 {
                    for want in 0..10u32 {
                        let key = Box::new(want);
                        let key_addr = addr(&*key);
                        let got = unsafe { fn_00AB6850::rw_00ab6850(this, key_addr, start) };
                        let want_out = array.find_from(want, start);
                        assert_eq!(got, want_out.unwrap_or(0xFFFF_FFFF));
                        if wrong::find_skip_first(&array, want, start) != want_out {
                            caught += 1;
                        }
                        cases += 1;
                    }
                }
            }
        }
        assert!(cases >= 100 && caught >= 1, "cases={cases} caught={caught}");
    }
    // Recording stubs for the match comparison and the reset pair.
    static CMP_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    static CMP_SCRIPT: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "cdecl" fn cmp_stub(entry: u32, key: u32) -> u32 {
        CMP_LOG.lock().unwrap().push((entry, key));
        CMP_SCRIPT.lock().unwrap().pop().unwrap_or(0)
    }
    static NOTIFY_LOG: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());
    extern "cdecl" fn notify_stub(idx: u32, code: u32) -> u32 {
        NOTIFY_LOG.lock().unwrap().push((idx, code));
        0
    }
    static RELEASE_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    extern "thiscall" fn release_stub(record: u32) -> u32 {
        RELEASE_LOG.lock().unwrap().push(record);
        0
    }

    /// One match instance under test.
    type MatchFn = extern "thiscall" fn(u32, u32, u32) -> u32;

    /// Runs a match instance over bank/flag/diff shapes.
    fn run_match(match_fn: MatchFn, bank: FlagBank, seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..8 {
            // At most ten slots: past fourteen the two banks alias.
            let n = 1 + (trial % 10) as usize;
            let mut flags_a = vec![];
            let mut flags_b = vec![];
            let mut entries = vec![];
            for i in 0..n {
                // Banks disagree everywhere so the bank pinning matters.
                let (fa, fb) = if trial == 0 {
                    (u8::from(i % 2 == 0), u8::from(i % 2 == 1))
                } else {
                    (rng.below(3) as u8, rng.below(3) as u8)
                };
                flags_a.push(fa);
                flags_b.push(fb);
                let mut entry = [0u8; 8];
                rng.bytes(&mut entry);
                entries.push(entry);
            }
            let block = ControlBlock {
                flags_a: flags_a.clone(),
                flags_b: flags_b.clone(),
                entries: entries.clone(),
            };
            let mut obj = vec![0u8; CTRL_ENTRY + n * CTRL_STRIDE].into_boxed_slice();
            rng.bytes(&mut obj);
            for (i, flag) in flags_a.iter().enumerate() {
                obj[BANK_A + i] = *flag;
            }
            for (i, flag) in flags_b.iter().enumerate() {
                obj[BANK_B + i] = *flag;
            }
            for (i, entry) in entries.iter().enumerate() {
                obj[CTRL_ENTRY + i * CTRL_STRIDE..CTRL_ENTRY + (i + 1) * CTRL_STRIDE]
                    .copy_from_slice(entry);
            }
            let this = addr(&obj[0]);
            for idx in 0..n as u32 {
                for &key in &[0u32, 1, 0x1234_5678] {
                    for &diff in &[0u32, 1, 0xFFFF_FFFF] {
                        CMP_SCRIPT.lock().unwrap().push(diff);
                        CMP_LOG.lock().unwrap().clear();
                        let got = unsafe { match_fn(this, key, idx) };
                        let log = CMP_LOG.lock().unwrap().clone();
                        let mut lift_calls: Vec<([u8; 8], u32)> = vec![];
                        let want = block.match_slot(key, idx, bank, &mut |entry, seen| {
                            lift_calls.push((entry, seen));
                            diff
                        });
                        assert_eq!(got, u32::from(want));
                        // The call log matches entry address and key.
                        let entry_addr = this
                            .wrapping_add(idx.wrapping_mul(CTRL_STRIDE as u32))
                            .wrapping_add(CTRL_ENTRY as u32);
                        if want || !log.is_empty() {
                            assert_eq!(log, vec![(entry_addr, key)]);
                            assert_eq!(lift_calls, vec![(entries[idx as usize], key)]);
                        } else {
                            assert!(log.is_empty());
                            assert!(lift_calls.is_empty());
                        }
                        // The wrong lift always calls; the other-bank
                        // lift reads the wrong flag.
                        let mut bad_calls = 0;
                        let bad = wrong::match_no_flag(&block, key, idx, diff, &mut bad_calls);
                        let mut other_calls = 0;
                        let other =
                            block.match_slot(key, idx, wrong::other_bank(bank), &mut |_, _| {
                                other_calls += 1;
                                diff
                            });
                        if bad != want || bad_calls as usize != log.len() || other != want {
                            caught += 1;
                        }
                        cases += 1;
                    }
                }
            }
        }
        (cases, caught)
    }

    #[test]
    fn match_f0_matches() {
        let _guard = lock();
        rt::set_callee(1, cmp_stub as *const () as usize as u32);
        let (cases, caught) = run_match(fn_008C5FF0::rw_008c5ff0, FlagBank::A, 0xF0);
        assert!(cases >= 100 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn match_fe_matches() {
        let _guard = lock();
        rt::set_callee(1, cmp_stub as *const () as usize as u32);
        let (cases, caught) = run_match(fn_008C6090::rw_008c6090, FlagBank::B, 0xFE);
        assert!(cases >= 100 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn control_reset_matches() {
        let _guard = lock();
        rt::set_callee(1, notify_stub as *const () as usize as u32);
        rt::set_callee(2, release_stub as *const () as usize as u32);
        let mut rng = Rng(0xC7C1);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..10 {
            let n = 1 + (trial % 10) as usize;
            let mut flags_a = vec![];
            let mut flags_b = vec![];
            let mut entries = vec![];
            for _ in 0..n {
                flags_a.push(rng.below(256) as u8);
                flags_b.push(rng.below(256) as u8);
                let mut entry = [0u8; 8];
                rng.bytes(&mut entry);
                entries.push(entry);
            }
            for idx in 0..n as u32 {
                let mut block = ControlBlock {
                    flags_a: flags_a.clone(),
                    flags_b: flags_b.clone(),
                    entries: entries.clone(),
                };
                let mut wrong_block = block.clone();
                let mut obj = vec![0u8; CTRL_ENTRY + n * CTRL_STRIDE].into_boxed_slice();
                rng.bytes(&mut obj);
                for (i, flag) in flags_a.iter().enumerate() {
                    obj[BANK_A + i] = *flag;
                }
                for (i, flag) in flags_b.iter().enumerate() {
                    obj[BANK_B + i] = *flag;
                }
                for (i, entry) in entries.iter().enumerate() {
                    obj[CTRL_ENTRY + i * CTRL_STRIDE..CTRL_ENTRY + (i + 1) * CTRL_STRIDE]
                        .copy_from_slice(entry);
                }
                let this = addr(&obj[0]);
                NOTIFY_LOG.lock().unwrap().clear();
                RELEASE_LOG.lock().unwrap().clear();
                let got = unsafe { fn_008C6030::rw_008c6030(this, idx) };
                assert_eq!(got, 0);
                let notify_log = NOTIFY_LOG.lock().unwrap().clone();
                let release_log = RELEASE_LOG.lock().unwrap().clone();
                let mut lift_notify = vec![];
                let mut lift_release = vec![];
                block.reset_slot(
                    idx,
                    &mut |seen_idx, code| lift_notify.push((seen_idx, code)),
                    &mut |offset| lift_release.push(offset),
                );
                // Calls in order with rebuilt addresses.
                assert_eq!(notify_log, vec![(idx, 1)]);
                assert_eq!(lift_notify, notify_log);
                let first = this.wrapping_add(idx.wrapping_add(2).wrapping_mul(8));
                let second = this.wrapping_add(idx.wrapping_add(0x10).wrapping_mul(8));
                assert_eq!(release_log, vec![first, second]);
                assert_eq!(
                    lift_release,
                    vec![
                        idx.wrapping_add(2).wrapping_mul(8),
                        idx.wrapping_add(0x10).wrapping_mul(8)
                    ]
                );
                // Flag byte cleared, entry zeroed, rest untouched.
                let back = read_back(this, CTRL_ENTRY + n * CTRL_STRIDE);
                assert_eq!(back[BANK_A + idx as usize], 0);
                assert_eq!(block.flags_a[idx as usize], 0);
                let at = CTRL_ENTRY + idx as usize * CTRL_STRIDE;
                assert_eq!(&back[at..at + CTRL_STRIDE], &[0; CTRL_STRIDE]);
                assert_eq!(block.entries[idx as usize], [0; 8]);
                // The wrong lift swaps the two releases.
                let mut bad_notify = vec![];
                let mut bad_releases = vec![];
                wrong::reset_swapped(&mut wrong_block, idx, &mut bad_notify, &mut bad_releases);
                if bad_releases != lift_release || wrong_block != block {
                    caught += 1;
                }
                cases += 1;
            }
        }
        assert!(cases >= 40 && caught >= 1, "cases={cases} caught={caught}");
    }
}
