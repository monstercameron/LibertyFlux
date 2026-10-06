//! Differential cases, part 4: record search and slot liveness.
//!
//! Each case builds real 32-bit objects, runs the rewrite and the lifted
//! method on the same inputs, and compares returns (rebuilt addresses
//! where the lift answers indexes) and every effect. Each method has a
//! deliberately wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_slotdiff::rewrites::*;
    use lf_slotdiff::rt;
    use lf_streaming::slots::RecordSet;
    use lf_streaming::slots::SlotHeader;
    use lf_streaming::slots::SlotLiveness;
    use lf_streaming::slots::WordTable;
    use lf_streaming::slots::SLOT_HEADER_LEN;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{addr, get_u32, lock, put_u32, Rng, USE_ROW_VA};

    /// Record strides of the two key scans.
    const FIND_STRIDE: usize = 0x28;
    const LOOKUP_STRIDE: usize = 0x50;
    /// Wanted-key offset past the find argument.
    const FIND_KEY_OFF: usize = 0x2c;
    /// Word-table field offsets.
    const WORDS_BASE: usize = 0x18;
    const WORDS_COUNT: usize = 0x1c;
    /// In-use row stride and slot count.
    const USE_STRIDE: usize = 0x134;
    const USE_COUNT: usize = 75;

    // Deliberately wrong lifts: each must be caught at least once per method.
    mod wrong {
        use lf_streaming::slots::RecordSet;
        use lf_streaming::slots::SlotHeader;
        use lf_streaming::slots::SlotLiveness;
        use lf_streaming::slots::WordTable;

        /// Answers 0 for every miss, forgetting the below residue.
        pub fn contains_no_residue(set: &RecordSet, want: &[u8]) -> u32 {
            let got = set.contains_key(want);
            if got == 0xFFFF_FF00 {
                0
            } else {
                got
            }
        }

        /// Reports the last match instead of the first.
        pub fn position_last(set: &RecordSet, want: &[u8]) -> Option<usize> {
            if i32::from(set.count) <= 0 {
                return None;
            }
            let mut found = None;
            for (i, key) in set.keys.iter().enumerate() {
                if key.as_slice() == want {
                    found = Some(i);
                }
            }
            found
        }

        /// Answers `None` for index 0.
        pub fn get_skip_zero(table: &WordTable, index: u32) -> Option<u32> {
            if index == 0 {
                return None;
            }
            table.get(index)
        }

        /// Reads the whole flag word instead of its low byte.
        pub fn free_full_flag(live: &SlotLiveness, index: u32, flag: u32) -> bool {
            let limit = if flag != 0 { 15 } else { 75 };
            if (index as i32) < 0 || index >= limit {
                return false;
            }
            live.used[index as usize] == 0
        }

        /// Clears the flag byte instead of setting it.
        pub fn init_flag_zero(header: &mut SlotHeader, value: u32) {
            header.init(value);
            header.flag = 0;
        }
    }

    /// Random key bytes without NUL, up to `max` long.
    fn random_key(rng: &mut Rng, max: usize) -> Vec<u8> {
        let len = rng.below(max as u32 + 1) as usize;
        let mut key = vec![0u8; len];
        for byte in key.iter_mut() {
            let mut value = 0;
            while value == 0 {
                value = rng.u32() as u8;
            }
            *byte = value;
        }
        key
    }

    /// Plants a record set with the given stride, returning the object
    /// address. Keys are NUL-terminated inside their records.
    fn plant_records(
        rng: &mut Rng,
        keys: &[Vec<u8>],
        count: u16,
        stride: usize,
    ) -> (Box<[u8]>, Box<[u8]>, u32) {
        let mut records = vec![0u8; keys.len().max(1) * stride].into_boxed_slice();
        rng.bytes(&mut records);
        for (i, key) in keys.iter().enumerate() {
            let at = i * stride;
            records[at..at + key.len()].copy_from_slice(key);
            records[at + key.len()] = 0;
        }
        let base = addr(&records[0]);
        let mut obj = vec![0u8; 8].into_boxed_slice();
        rng.bytes(&mut obj);
        put_u32(&mut obj, 0, base);
        obj[4..6].copy_from_slice(&count.to_le_bytes());
        let this = addr(&obj[0]);
        (records, obj, this)
    }

    #[test]
    fn record_find_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF1D);
        let mut cases = 0;
        let mut caught = 0;
        // Key shapes: empty, prefixes, long, high bytes.
        let mut shape_keys = vec![
            vec![],
            b"a".to_vec(),
            b"app".to_vec(),
            b"apple".to_vec(),
            b"z".to_vec(),
            vec![0xFF; 39],
        ];
        for _ in 0..6 {
            shape_keys.push(random_key(&mut rng, 39));
        }
        for trial in 0..8 {
            let n = (trial % 5) as usize;
            let keys: Vec<Vec<u8>> = shape_keys.iter().cycle().take(n).cloned().collect();
            // Counts past the owned keys are out of domain (the lift
            // panics; the rewrite reads on through wild memory).
            for &count in &[n as u16, 0u16, n.saturating_sub(1) as u16] {
                let set = RecordSet {
                    keys: keys.clone(),
                    count,
                };
                let (_records, _obj, this) = plant_records(&mut rng, &keys, count, FIND_STRIDE);
                // Wanted keys: present, absent above/below, empty.
                let mut wants = vec![vec![], b"app".to_vec(), b"apricot".to_vec()];
                for key in keys.iter().take(3) {
                    wants.push(key.clone());
                }
                wants.push(random_key(&mut rng, 39));
                for want in &wants {
                    let mut arg = vec![0u8; FIND_KEY_OFF + want.len() + 1].into_boxed_slice();
                    rng.bytes(&mut arg);
                    arg[FIND_KEY_OFF..FIND_KEY_OFF + want.len()].copy_from_slice(want);
                    arg[FIND_KEY_OFF + want.len()] = 0;
                    let arg_addr = addr(&arg[0]);
                    let got = unsafe { fn_00C07820::rw_00c07820(this, arg_addr) };
                    assert_eq!(got, set.contains_key(want));
                    if wrong::contains_no_residue(&set, want) != got {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases >= 100 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn record_lookup_matches() {
        let _guard = lock();
        let mut rng = Rng(0x100);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..8 {
            let n = (trial % 6) as usize;
            // Small key alphabet so matches repeat.
            let alphabet: Vec<Vec<u8>> = vec![
                vec![],
                b"k0".to_vec(),
                b"k1".to_vec(),
                b"k2".to_vec(),
                b"longer-key-here".to_vec(),
            ];
            // Three live keys over up to six slots, so matches repeat.
            let keys: Vec<Vec<u8>> = (0..n).map(|i| alphabet[(i + trial) % 3].clone()).collect();
            // Counts past the owned keys are out of domain (the lift
            // panics; the rewrite reads on through wild memory).
            for &count in &[n as u16, 0u16, n.saturating_sub(1) as u16] {
                let set = RecordSet {
                    keys: keys.clone(),
                    count,
                };
                let (_records, _obj, this) = plant_records(&mut rng, &keys, count, LOOKUP_STRIDE);
                let base = get_u32(
                    unsafe { std::slice::from_raw_parts(this as *const u8, 8) },
                    0,
                );
                let mut wants: Vec<Vec<u8>> = alphabet.clone();
                wants.push(b"absent".to_vec());
                for want in &wants {
                    let mut key_buf = vec![0u8; want.len() + 1].into_boxed_slice();
                    key_buf[..want.len()].copy_from_slice(want);
                    let key_addr = addr(&key_buf[0]);
                    let got = unsafe { fn_00C078C0::rw_00c078c0(this, key_addr) };
                    let want_out = set.position(want);
                    let rebuilt = want_out.map_or(0, |i| {
                        base.wrapping_add((i as u32).wrapping_mul(LOOKUP_STRIDE as u32))
                    });
                    assert_eq!(got, rebuilt);
                    let bad = wrong::position_last(&set, want);
                    let bad_rebuilt = bad.map_or(0, |i| {
                        base.wrapping_add((i as u32).wrapping_mul(LOOKUP_STRIDE as u32))
                    });
                    if bad_rebuilt != got {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases >= 100 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn table_get_matches() {
        let _guard = lock();
        let mut rng = Rng(0x6E7);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..10 {
            let n = (trial % 5) as usize;
            let mut words = vec![];
            for _ in 0..n {
                words.push(rng.u32());
            }
            if trial == 0 && !words.is_empty() {
                words[0] = 0; // a stored zero answers Some(0).
            }
            for &count in &[n as u16, 0u16] {
                let table = WordTable {
                    words: words.clone(),
                    count,
                };
                let mut entries = vec![0u8; n.max(1) * 4].into_boxed_slice();
                for (i, word) in words.iter().enumerate() {
                    put_u32(&mut entries, i * 4, *word);
                }
                let base = addr(&entries[0]);
                let mut obj = vec![0u8; WORDS_COUNT + 2].into_boxed_slice();
                rng.bytes(&mut obj);
                put_u32(&mut obj, WORDS_BASE, base);
                obj[WORDS_COUNT..WORDS_COUNT + 2].copy_from_slice(&count.to_le_bytes());
                let this = addr(&obj[0]);
                let idxs = [0u32, 1, n as u32, n as u32 + 1, 0x7FFF_FFFF];
                for &index in &idxs {
                    let got = unsafe { fn_00C07A60::rw_00c07a60(this, index) };
                    let want = table.get(index);
                    assert_eq!(got, want.unwrap_or(0));
                    // Missing and stored-zero both read 0: the index
                    // check, not the value, tells them apart.
                    assert_eq!(want.is_some(), (index as i32) < i32::from(count));
                    if wrong::get_skip_zero(&table, index) != want {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases >= 60 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn slot_free_matches() {
        let _guard = lock();
        let mut rng = Rng(0xF8EE);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..6 {
            let mut used = vec![0u8; USE_COUNT];
            if trial == 0 {
                // All free: every in-range index passes.
            } else if trial == 1 {
                used = vec![1u8; USE_COUNT];
            } else {
                rng.bytes(&mut used);
            }
            let live = SlotLiveness { used: used.clone() };
            let mut row = vec![0u8; (USE_COUNT - 1) * USE_STRIDE + 1].into_boxed_slice();
            rng.bytes(&mut row);
            for (i, byte) in used.iter().enumerate() {
                row[i * USE_STRIDE] = *byte;
            }
            rt::set_relocated(USE_ROW_VA, addr(&row[0]));
            let flags = [0u32, 1, 0xFF, 0x100, 0x101, 0xFF00];
            let idxs = [
                0u32,
                1,
                14,
                15,
                16,
                74,
                75,
                76,
                0x7FFF_FFFF,
                0x8000_0000,
                0xFFFF_FFFF,
            ];
            for &flag in &flags {
                for &index in &idxs {
                    let got = unsafe { fn_008CB3A0::rw_008CB3A0(index, flag) };
                    assert_eq!(got, u32::from(live.is_free(index, flag)));
                    if u32::from(wrong::free_full_flag(&live, index, flag)) != got {
                        caught += 1;
                    }
                    cases += 1;
                }
            }
        }
        assert!(cases >= 200 && caught >= 1, "cases={cases} caught={caught}");
    }

    #[test]
    fn slot_init_matches() {
        let _guard = lock();
        let mut rng = Rng(0x11);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..40 {
            let mut bytes = [0u8; SLOT_HEADER_LEN];
            rng.bytes(&mut bytes);
            if trial == 0 {
                bytes = [0xFF; SLOT_HEADER_LEN];
            }
            let value = rng.u32();
            let mut planted = vec![0u8; SLOT_HEADER_LEN].into_boxed_slice();
            planted.copy_from_slice(&bytes);
            let this = addr(&planted[0]);
            let got = unsafe { fn_008CBF30::rw_008CBF30(this, value) };
            assert_eq!(got, 0, "trial {trial}");
            let mut lift = SlotHeader::from_bytes(bytes);
            lift.init(value);
            let back = unsafe { std::slice::from_raw_parts(this as *const u8, SLOT_HEADER_LEN) };
            assert_eq!(back, lift.to_bytes(), "trial {trial}");
            let mut bad = SlotHeader::from_bytes(bytes);
            wrong::init_flag_zero(&mut bad, value);
            if bad.to_bytes() != lift.to_bytes() {
                caught += 1;
            }
            cases += 1;
        }
        assert!(cases >= 40 && caught >= 1, "cases={cases} caught={caught}");
    }
}
