//! Differential cases, part 4: the tag search and the bounded search.
//!
//! Both routines are pure (no globals, no callees): each case builds the
//! 32-bit object, runs the rewrite and the lifted search on the same
//! inputs, and compares the index and every written word. Each search has
//! a deliberately wrong lift that must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use lf_pooldiff::rewrites::*;
    use lf_pooldiff::rt;
    use lf_world::pools::{KeyedFlags, PairPool, TagPool, TagPools, WordBlocks, WordTable};

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock, put_u32};

    /// Pool base offsets within the search object, as the rewrite holds them.
    const POOLS: [u32; 6] = [0, 0xF70, 0x7B8, 0x1EE0, 0x1728, 0x32AC];
    /// Tag base offset within the object; tags sit 16 bytes apart per key.
    const TAG_BASE: u32 = 0x2698;
    /// Tag column: row `i`'s tag is word `i + 0x14A` of its pool.
    const TAG_COL: u32 = 0x14A;

    /// Deliberately wrong lifts, each caught below.
    mod wrong {
        use lf_world::pools::{TagPools, WordTable};

        /// Forgets the 24-bit masking of the first payload.
        pub fn find_unmasked(
            pools: &TagPools,
            kind: u32,
            key: u32,
            start: u32,
            out0: &mut u32,
            out1: &mut u32,
        ) -> Option<usize> {
            let pool = pools.pools.get(kind as usize)?;
            let mut i = start as usize;
            while i < pool.tags.len() {
                if pool.tags[i] == key {
                    *out0 = pool.w0[i];
                    *out1 = pool.w1[i];
                    return Some(i);
                }
                i += 1;
            }
            None
        }

        /// Reports the last match in the slice instead of the first.
        pub fn search_last(table: &WordTable, want: u32, start: i32, count: i32) -> Option<usize> {
            let limit = table.words.len() as i32;
            if start < 0 || start >= limit {
                return None;
            }
            if count < 0 {
                return None;
            }
            let end = start.wrapping_add(count);
            if end > limit || start >= end {
                return None;
            }
            let mut found = None;
            let mut i = start;
            while i < end {
                if table.words[i as usize] == want {
                    found = Some(i as usize);
                }
                i += 1;
            }
            found
        }
    }

    #[test]
    fn entry_find_matches() {
        let _guard = lock();
        let mut rng = Rng(0xEF1D);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..40 {
            // Six pools of 0..8 rows with random keys and payloads.
            let mut pools: Vec<TagPool> = Vec::new();
            for _ in 0..6 {
                let n = rng.below(9) as usize;
                let (mut tags, mut w0, mut w1) = (vec![], vec![], vec![]);
                for _ in 0..n {
                    tags.push(rng.u32());
                    w0.push(rng.u32());
                    w1.push(rng.u32());
                }
                // Trial 0 pins the top-byte masking with fixed payloads.
                if trial == 0 && !w0.is_empty() {
                    w0[0] = 0xFF00_0000 | (trial as u32);
                }
                pools.push(TagPool::from_rows(tags, w0, w1));
            }
            let lift_pools = TagPools {
                pools: pools.clone().try_into().unwrap(),
            };
            // The 32-bit object: counts, tag columns, payload columns.
            // Boxed first so the tags (object-relative addresses) are exact.
            let obj_box: Box<[u8]> = vec![0u8; 16384].into_boxed_slice();
            let this = addr(&obj_box[0]);
            let mut raw = unsafe { std::slice::from_raw_parts_mut(this as *mut u8, 16384) };
            for (kind, pool) in pools.iter().enumerate() {
                let base = POOLS[kind] as usize;
                put_u32(&mut raw, base, pool.tags.len() as u32);
                for (i, key) in pool.tags.iter().enumerate() {
                    let tag = this
                        .wrapping_add(TAG_BASE)
                        .wrapping_add(key.wrapping_shl(4));
                    put_u32(&mut raw, base + ((i as u32 + TAG_COL) * 4) as usize, tag);
                    put_u32(&mut raw, base + i * 4 + 0x298, pool.w0[i]);
                    put_u32(&mut raw, base + i * 4 + 8, pool.w1[i]);
                }
            }
            // Searches: each pool, hits and misses, starts around the count.
            for kind in 0..6u32 {
                let n = pools[kind as usize].tags.len() as u32;
                let mut keys = vec![0u32, 1, u32::MAX, rng.u32()];
                for k in pools[kind as usize].tags.iter() {
                    keys.push(*k);
                    keys.push(k.wrapping_add(1));
                    keys.push(k.wrapping_sub(1));
                }
                for &key in &keys {
                    for start in [0u32, 1, n.saturating_sub(1), n, n.wrapping_add(1), 7, 100] {
                        let mut out_rw = [0xA11CEu32, 0xB0Bu32];
                        let (mut out0, mut out1) = (0xA11CEu32, 0xB0Bu32);
                        let rw_out0 = addr(&out_rw[0]);
                        // See part 2: pass and read raw words.
                        let got = unsafe {
                            fn_008EFCB0::rw_008efcb0(
                                this,
                                kind,
                                key,
                                rw_out0,
                                rw_out0.wrapping_add(4),
                                start,
                            )
                        };
                        out_rw[0] = unsafe { (rw_out0 as *const u32).read_unaligned() };
                        out_rw[1] =
                            unsafe { (rw_out0.wrapping_add(4) as *const u32).read_unaligned() };
                        let lift = lift_pools.find(kind, key, start, &mut out0, &mut out1);
                        assert_eq!(
                            got,
                            lift.map_or(0xFFFF_FFFF, |i| i as u32),
                            "kind={kind} key={key:#x} start={start}"
                        );
                        assert_eq!(out_rw, [out0, out1], "payload words must match");
                        let (mut w0, mut w1) = (0xA11CEu32, 0xB0Bu32);
                        let wl =
                            wrong::find_unmasked(&lift_pools, kind, key, start, &mut w0, &mut w1);
                        if wl != lift || [w0, w1] != [out0, out1] {
                            caught += 1;
                        }
                        cases += 1;
                    }
                }
            }
            // Unknown kinds answer -1 without touching the outputs.
            for kind in [6u32, 7, 100, u32::MAX] {
                let mut out_rw = [0xA11CEu32, 0xB0Bu32];
                let rw_out0 = addr(&out_rw[0]);
                let got = unsafe {
                    fn_008EFCB0::rw_008efcb0(
                        this,
                        kind,
                        0x1234,
                        rw_out0,
                        rw_out0.wrapping_add(4),
                        0,
                    )
                };
                out_rw[0] = unsafe { (rw_out0 as *const u32).read_unaligned() };
                let (mut out0, mut out1) = (0xA11CEu32, 0xB0Bu32);
                let lift = lift_pools.find(kind, 0x1234, 0, &mut out0, &mut out1);
                assert_eq!(got, 0xFFFF_FFFF);
                assert_eq!(lift, None);
                assert_eq!(out_rw, [0xA11CE, 0xB0B]);
                assert_eq!([out0, out1], [0xA11CE, 0xB0B]);
                cases += 1;
            }
            std::hint::black_box(&obj_box);
        }
        assert!(cases > 1000, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong masking never caught ({cases} cases)");
    }

    #[test]
    fn index_search_matches() {
        let _guard = lock();
        let mut rng = Rng(0x1D3);
        let mut cases = 0;
        let mut caught = 0;
        for &n in &[0usize, 1, 2, 3, 5, 8, 16, 33, 64] {
            let mut words = vec![0u32; n];
            for w in words.iter_mut() {
                *w = rng.u32();
            }
            // A repeated value pins first-match reporting.
            if n >= 4 {
                words[n - 1] = words[0];
            }
            let table = WordTable {
                words: words.clone(),
            };
            let words_box = words.into_boxed_slice();
            let base = if n == 0 { 0 } else { addr(&words_box[0]) };
            let desc = Box::new([base, n as u32]);
            let this = addr(&desc[0]);
            let bounds = [
                i32::MIN,
                i32::MIN + 1,
                -3,
                -2,
                -1,
                0,
                1,
                2,
                n as i32 - 1,
                n as i32,
                n as i32 + 1,
                n as i32 + 2,
                i32::MAX - 1,
                i32::MAX,
            ];
            let mut wants = vec![0u32, 1, u32::MAX, rng.u32(), rng.u32()];
            for &w in table.words.iter() {
                wants.push(w);
            }
            // Seeded needles, one address per value.
            let needles: Vec<Box<u32>> = wants.iter().map(|&w| Box::new(w)).collect();
            for (ni, &want) in wants.iter().enumerate() {
                let needle = addr(&*needles[ni]);
                for &start in &bounds {
                    for &count in &bounds {
                        let got = unsafe {
                            fn_009D1FF0::rw_009D1FF0(this, needle, start as u32, count as u32)
                        };
                        let lift = table.search(want, start, count);
                        assert_eq!(
                            got,
                            lift.map_or(0xFFFF_FFFF, |i| i as u32),
                            "n={n} want={want:#x} start={start} count={count}"
                        );
                        if wrong::search_last(&table, want, start, count) != lift {
                            caught += 1;
                        }
                        cases += 1;
                    }
                }
            }
            std::hint::black_box((&desc, &words_box, &needles));
        }
        assert!(cases > 1000, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong last-match never caught ({cases} cases)");
    }

    // Element-initialiser stub for the pair creation: scripted answers.
    static ELEM_LOG: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    static ELEM_SCRIPT: Mutex<VecDeque<u32>> = Mutex::new(VecDeque::new());
    extern "thiscall" fn elem_stub(elem: u32) -> u32 {
        ELEM_LOG.lock().unwrap().push(elem);
        ELEM_SCRIPT.lock().unwrap().pop_front().unwrap_or(0)
    }

    #[test]
    fn pair_init_matches() {
        let _guard = lock();
        ELEM_LOG.lock().unwrap().clear();
        ELEM_SCRIPT.lock().unwrap().clear();
        rt::set_callee(1, elem_stub as usize as u32);
        let mut rng = Rng(0x9A12);
        let mut cases = 0;
        let mut caught = 0;
        for _ in 0..32 {
            let a1 = rng.u32();
            let a2 = a1 ^ 0x00FF_FFFF;
            let mut obj = vec![0u8; 0x1921];
            rng.bytes(&mut obj);
            obj[0x1920] = 0x5A;
            let obj_box = obj.into_boxed_slice();
            let base = addr(&obj_box[0]);
            ELEM_SCRIPT.lock().unwrap().push_back(a1);
            ELEM_SCRIPT.lock().unwrap().push_back(a2);
            let before = ELEM_LOG.lock().unwrap().len();
            let got = unsafe { fn_00946C00::rw_00946c00(base as *mut u8) };
            let rw_log = ELEM_LOG.lock().unwrap()[before..].to_vec();
            assert_eq!(got, a2, "the second answer wins");
            assert_eq!(rw_log, [base, base.wrapping_add(0xBD0)]);
            let ready = unsafe { (base.wrapping_add(0x1920) as *const u8).read() };
            assert_eq!(ready, 1, "the ready byte is raised");
            // The lift threads token elements through in order.
            let mut lift_log: Vec<u32> = Vec::new();
            let mut script = VecDeque::from([a1, a2]);
            let (pool, answer) = PairPool::init(0xE0u32, 0xE1u32, &mut |elem: &mut u32| {
                lift_log.push(*elem);
                script.pop_front().unwrap()
            });
            assert_eq!(answer, got);
            assert_eq!(lift_log, [0xE0, 0xE1]);
            assert!(pool.ready);
            assert_eq!((pool.first, pool.second), (0xE0, 0xE1));
            // Wrong: answer the first call.
            if a1 != answer {
                caught += 1;
            }
            cases += 1;
            std::hint::black_box(&obj_box);
        }
        assert!(cases >= 32, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong first-answer never caught ({cases} cases)"
        );
    }

    #[test]
    fn contains_value_matches() {
        let _guard = lock();
        let mut rng = Rng(0xC07A);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..24 {
            let mut blocks = [[0u32; 4]; 4];
            for b in blocks.iter_mut() {
                for w in b.iter_mut() {
                    *w = rng.u32();
                }
            }
            // Trial 0 pins the last block with a fixed marker.
            if trial == 0 {
                blocks = [[0u32; 4]; 4];
                blocks[3][2] = 7;
            }
            let lift = WordBlocks { blocks };
            let mut obj = vec![0u8; 4 * 0x80];
            for k in 0..4 {
                for j in 0..4 {
                    put_u32(&mut obj, 4 + k * 0x80 + j * 0x20, blocks[k][j]);
                }
            }
            let obj_box = obj.into_boxed_slice();
            let base = addr(&obj_box[0]);
            let mut values = vec![0u32, 1, u32::MAX, rng.u32()];
            for b in &blocks {
                for &w in b {
                    values.push(w);
                }
            }
            for &value in &values {
                let got = unsafe { fn_00948740::rw_00948740(base as *const u8, value) };
                let found = lift.contains(value);
                assert_eq!(got, u32::from(found), "value={value:#x}");
                // Wrong: skip the last block.
                let wrong = blocks[..3].iter().any(|b| b.contains(&value));
                if wrong != found {
                    caught += 1;
                }
                cases += 1;
            }
            std::hint::black_box(&obj_box);
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong skipped block never caught ({cases} cases)"
        );
    }

    #[test]
    fn clear_match_flags_matches() {
        let _guard = lock();
        let mut rng = Rng(0xC1EA);
        let mut cases = 0;
        let mut caught = 0;
        for trial in 0..16 {
            let mut keys = [0u32; 16];
            let mut flags = [0u8; 16];
            for i in 0..16 {
                keys[i] = rng.u32();
                flags[i] = rng.u32() as u8;
            }
            // Trial 0 pins the clearing with a fixed key and set flags.
            if trial == 0 {
                keys = [0u32; 16];
                keys[5] = 7;
                flags = [0xFFu8; 16];
            }
            let mut values = vec![0u32, u32::MAX, rng.u32()];
            for &k in &keys {
                values.push(k);
            }
            for &value in &values {
                let mut obj = vec![0u8; 16 * 0x40];
                for i in 0..16 {
                    put_u32(&mut obj, i * 0x40, keys[i]);
                    obj[i * 0x40 + 0x2C] = flags[i];
                }
                let obj_box = obj.into_boxed_slice();
                let base = addr(&obj_box[0]);
                let got = unsafe { fn_00949890::rw_00949890(base as *mut u8, value) };
                assert_eq!(got, 0);
                let mut rw_flags = [0u8; 16];
                for i in 0..16 {
                    rw_flags[i] = unsafe {
                        (base.wrapping_add((i * 0x40 + 0x2C) as u32) as *const u8).read()
                    };
                    let rw_key = unsafe {
                        (base.wrapping_add((i * 0x40) as u32) as *const u32).read_unaligned()
                    };
                    assert_eq!(rw_key, keys[i], "keys are untouched");
                }
                let mut lifted = KeyedFlags { keys, flags };
                lifted.clear_matches(value);
                assert_eq!(rw_flags, lifted.flags, "value={value:#x}");
                // Wrong: clear on mismatch.
                let mut wflags = flags;
                for i in 0..16 {
                    if keys[i] != value {
                        wflags[i] = 0;
                    }
                }
                if wflags != lifted.flags {
                    caught += 1;
                }
                cases += 1;
                std::hint::black_box(&obj_box);
            }
        }
        assert!(cases > 200, "too few comparisons ({cases})");
        assert!(
            caught > 0,
            "wrong inverted clear never caught ({cases} cases)"
        );
    }
}
