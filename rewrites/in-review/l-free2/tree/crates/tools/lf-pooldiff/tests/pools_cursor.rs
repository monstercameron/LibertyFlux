//! Differential cases, part 1: the cursor step against its seven instances.
//!
//! Each case plants one record global, runs the rewrite and
//! [`SlotPool::cursor_step`] on the same cursor, and compares the return
//! (reconstructed as `base + stride * index`), the cursor after the call,
//! and the answered index. A deliberately wrong lift (scanning upward
//! instead of downward) must be caught. 32-bit target only.

#![allow(unsafe_code)]
// Rewrite calls keep explicit unsafe blocks (raw addresses flow through them)
// even though the included exports are declared safe.
#![allow(unused_unsafe)]

#[cfg(target_arch = "x86")]
mod x86 {
    use lf_pooldiff::rewrites::*;
    use lf_pooldiff::rt;
    use lf_world::pools::SlotPool;

    #[path = "../support/mod.rs"]
    mod support;
    use support::{Rng, addr, lock};

    /// One cursor-step instance by address, matching [`support::REC_VAS`].
    type StepFn = extern "thiscall" fn(*mut i32) -> u32;
    const INSTANCES: [StepFn; 7] = [
        fn_00A31E40::rw_00a31e40,
        fn_00A31E90::rw_00a31e90,
        fn_00A31EE0::rw_00a31ee0,
        fn_00A31F30::rw_00a31f30,
        fn_00A31F80::rw_00a31f80,
        fn_00A31FD0::rw_00a31fd0,
        fn_00A32020::rw_00a32020,
    ];

    /// Deliberately wrong lift: scans upward from slot 0 instead of
    /// downward from the cursor. Must be caught whenever the topmost and
    /// bottommost live slots in range differ.
    fn wrong_scan_up(pool: &SlotPool, cursor: i32) -> Option<usize> {
        let top = if cursor <= -1 {
            pool.slot_count() as i32
        } else {
            cursor
        };
        if top < 1 {
            return None;
        }
        (0..top as usize).find(|&i| pool.flags()[i] & 0x80 == 0)
    }

    /// Runs one instance over cursor/flag-shape combinations.
    /// Returns (comparisons, wrong-caught).
    fn run_instance(step: StepFn, rec_va: u32, seed: u32) -> (u32, u32) {
        let mut rng = Rng(seed);
        let mut cases = 0;
        let mut caught = 0;
        // (slots, stride, flag-pattern, cursors) shapes.
        for &(n, stride) in &[
            (0usize, 16u32),
            (1, 4),
            (1, 16),
            (2, 8),
            (3, 12),
            (5, 16),
            (8, 4),
            (8, 20),
            (9, 32),
            (16, 16),
        ] {
            // Flag patterns: all live, all dead, alternating from each
            // end, single live slots, bit-6-only vs bit-7 mixes, random.
            let mut patterns: Vec<Vec<u8>> = vec![
                vec![0x00; n],
                vec![0x80; n],
                (0..n)
                    .map(|i| if i % 2 == 0 { 0x00 } else { 0x80 })
                    .collect(),
                (0..n)
                    .map(|i| if i % 2 == 0 { 0x80 } else { 0x00 })
                    .collect(),
                vec![0x40; n],
                (0..n)
                    .map(|i| if i % 3 == 0 { 0xC0 } else { 0x01 })
                    .collect(),
            ];
            for single in 0..n {
                let mut p = vec![0x80; n];
                p[single] = 0x00;
                patterns.push(p);
            }
            for _ in 0..4 {
                let mut p = vec![0u8; n];
                rng.bytes(&mut p);
                patterns.push(p);
            }
            for flags in &patterns {
                let entries = vec![0xA5u8; n * stride as usize];
                let pool = SlotPool::from_parts(entries.clone(), flags.clone(), stride);
                // The rewrite's record: base, flag bytes, count, stride.
                let entries_box = entries.into_boxed_slice();
                let flags_box = flags.clone().into_boxed_slice();
                let base = if n == 0 { 0 } else { addr(&entries_box[0]) };
                let flag_addr = if n == 0 { 0 } else { addr(&flags_box[0]) };
                let record = Box::new([base, flag_addr, n as u32, stride]);
                unsafe { rt::global::<u32>(rec_va).write(addr(&record[0])) };
                // Cursors: below, restart, bottom, middle, top, count.
                let mut cursors = vec![-3i32, -2, -1, 0, 1, n as i32 - 1, n as i32];
                for _ in 0..4 {
                    cursors.push(rng.u32() as i32 % (n as i32 + 3) - 2);
                }
                for &c0 in &cursors {
                    if c0 > n as i32 {
                        continue; // past the store: the lift panics (host-pinned).
                    }
                    let mut c_rw = c0;
                    let mut c_lift = c0;
                    let got = unsafe { step(&mut c_rw) };
                    let lift = pool.cursor_step(&mut c_lift);
                    assert_eq!(c_rw, c_lift, "n={n} cursor={c0} flags={flags:?}");
                    match lift {
                        Some(idx) => {
                            let expect =
                                base.wrapping_add((stride as i32).wrapping_mul(idx as i32) as u32);
                            assert_ne!(
                                expect, 0,
                                "null-slot guard would fire: case outside the proof domain"
                            );
                            assert_eq!(got, expect, "n={n} cursor={c0} flags={flags:?}");
                        }
                        None => assert_eq!(got, 0, "n={n} cursor={c0} flags={flags:?}"),
                    }
                    if wrong_scan_up(&pool, c0) != lift {
                        caught += 1;
                    }
                    cases += 1;
                    // Keep the record alive past the rewrite call.
                    std::hint::black_box((&record, &entries_box, &flags_box));
                }
            }
        }
        (cases, caught)
    }

    #[test]
    fn cursor_step_16f7d60_matches() {
        let _guard = lock();
        let (cases, caught) = run_instance(INSTANCES[0], support::REC_VAS[0], 0xC001);
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong scan never caught ({cases} cases)");
    }

    #[test]
    fn cursor_step_12bd0e8_matches() {
        let _guard = lock();
        let (cases, caught) = run_instance(INSTANCES[1], support::REC_VAS[1], 0xC002);
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong scan never caught ({cases} cases)");
    }

    #[test]
    fn cursor_step_166d9ec_matches() {
        let _guard = lock();
        let (cases, caught) = run_instance(INSTANCES[2], support::REC_VAS[2], 0xC003);
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong scan never caught ({cases} cases)");
    }

    #[test]
    fn cursor_step_18b6f10_matches() {
        let _guard = lock();
        let (cases, caught) = run_instance(INSTANCES[3], support::REC_VAS[3], 0xC004);
        assert!(cases > 500, "too few comparisons ({cases})");
        assert!(caught > 0, "wrong scan never caught ({cases} cases)");
    }

    #[test]
    fn cursor_step_1632c60_matches() {
        let _guard = lock();
        let (cases, caught) = run_instance(INSTANCES[4], support::REC_VAS[4], 0xC005);
        assert!(cases > 500, "too few comparisons ({cases} cases)");
        assert!(caught > 0, "wrong scan never caught ({cases} cases)");
    }

    #[test]
    fn cursor_step_18b6f1c_matches() {
        let _guard = lock();
        let (cases, caught) = run_instance(INSTANCES[5], support::REC_VAS[5], 0xC006);
        assert!(cases > 500, "too few comparisons ({cases} cases)");
        assert!(caught > 0, "wrong scan never caught ({cases} cases)");
    }

    #[test]
    fn cursor_step_12e22a4_matches() {
        let _guard = lock();
        let (cases, caught) = run_instance(INSTANCES[6], support::REC_VAS[6], 0xC007);
        assert!(cases > 500, "too few comparisons ({cases} cases)");
        assert!(caught > 0, "wrong scan never caught ({cases} cases)");
    }
}
