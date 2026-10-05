// original: 0x00D8BC90 audio_voice_scan (proposed)

/// Scan two voice-table limits, probing each row with a filter pair, and count
/// how many probes report a hit, stopping early once more than eight do.
///
/// There are no arguments. Two global counters bound the scan: `COUNT_LO`
/// rows ending at index `COUNT_LO - 1` down to 0, then rows from
/// `COUNT_HI - 1` down to `COUNT_LO`. Each row lives `ROW_STRIDE` bytes
/// apart from `ROW_BASE`. Every row is probed by calling the tune filter
/// (callee 7, two fixed float arguments, which differ between the two
/// passes) and then the match filter (callee 8, two fixed float arguments
/// plus the row pointer and a zero); the match filter's return value is
/// added to a running hit total. Before the scan, six one-shot setup calls
/// run (callees 1-6, fixed constant arguments).
///
/// The scan stops early with `index + 1` as soon as the hit total exceeds
/// `HIT_LIMIT` (8); a total of exactly 8 after the first pass also stops it.
/// Otherwise the result is the low counter. All loop and limit comparisons
/// are signed, and the hit total wraps.
///
/// Original: 0x00D8BC90 (cdecl, no arguments, eight callees, all cdecl).
lf_checker_rt::export!(cdecl, rw_00d8bc90() -> u32 {
    unsafe {
        const COUNT_LO: u32 = 0x0179bf88;
        const COUNT_HI: u32 = 0x0179bf8c;
        const ROW_BASE: u32 = 0x01797f6c;
        const ROW_STRIDE: u32 = 0x804;
        const HIT_LIMIT: i32 = 8;
        // Setup filter constants, first argument first.
        const SETUP_A1: u32 = 0x3f4ccccd;
        const SETUP_A2: u32 = 0x3f19999a;
        const FIRST_PASS_TUNE_A: u32 = 0x3f70a3d7;
        const FIRST_PASS_TUNE_B: u32 = 0x3f733333;
        const FIRST_PASS_MATCH_A: u32 = 0x3f70a3d7;
        const FIRST_PASS_MATCH_B: u32 = 0x3f6147ae;
        const SECOND_PASS_TUNE_A: u32 = 0x3e947ae1;
        const SECOND_PASS_TUNE_B: u32 = 0x3f733333;
        const SECOND_PASS_MATCH_A: u32 = 0x3e947ae1;
        const SECOND_PASS_MATCH_B: u32 = 0x3f6147ae;
        const SIGN_FLAG_ARG: u32 = 0x80000000;

        #[inline(always)]
        unsafe fn rd_global(file_va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(file_va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn row_ptr(index: u32) -> u32 {
            index.wrapping_mul(ROW_STRIDE).wrapping_add(lf_checker_rt::relocated(ROW_BASE))
        }

        // Six one-shot setup calls with fixed constant arguments.
        lf_checker_rt::callee_cdecl!(1, u32, 0u32, 0u32);
        lf_checker_rt::callee_cdecl!(2, u32, 1u32);
        lf_checker_rt::callee_cdecl!(3, u32, 0u32);
        lf_checker_rt::callee_cdecl!(4, u32, 1u32);
        lf_checker_rt::callee_cdecl!(5, u32, SETUP_A1, SETUP_A2);
        lf_checker_rt::callee_cdecl!(6, u32, SIGN_FLAG_ARG);

        let mut hits: u32 = 0;
        let mut lo: u32 = rd_global(COUNT_LO);
        let mut index: u32 = lo.wrapping_sub(1);
        if (index as i32) >= 0 {
            let mut row = row_ptr(index);
            loop {
                lf_checker_rt::callee_cdecl!(7, u32, FIRST_PASS_TUNE_A, FIRST_PASS_TUNE_B);
                let got: u32 =
                    lf_checker_rt::callee_cdecl!(8, u32, FIRST_PASS_MATCH_A, FIRST_PASS_MATCH_B, row, 0u32);
                hits = hits.wrapping_add(got);
                if (hits as i32) > HIT_LIMIT {
                    return index.wrapping_add(1);
                }
                row = row.wrapping_sub(ROW_STRIDE);
                index = index.wrapping_sub(1);
                if (index as i32) < 0 {
                    break;
                }
            }
            if (hits as i32) >= HIT_LIMIT {
                return index.wrapping_add(1);
            }
            lo = rd_global(COUNT_LO);
        }
        index = rd_global(COUNT_HI).wrapping_sub(1);
        if (index as i32) < (lo as i32) {
            return if (hits as i32) <= HIT_LIMIT { lo } else { index.wrapping_add(1) };
        }
        let mut row = row_ptr(index);
        loop {
            lf_checker_rt::callee_cdecl!(7, u32, SECOND_PASS_TUNE_A, SECOND_PASS_TUNE_B);
            let got: u32 =
                lf_checker_rt::callee_cdecl!(8, u32, SECOND_PASS_MATCH_A, SECOND_PASS_MATCH_B, row, 0u32);
            hits = hits.wrapping_add(got);
            if (hits as i32) > HIT_LIMIT {
                return index.wrapping_add(1);
            }
            lo = rd_global(COUNT_LO);
            index = index.wrapping_sub(1);
            row = row.wrapping_sub(ROW_STRIDE);
            if (index as i32) < (lo as i32) {
                break;
            }
        }
        if (hits as i32) <= HIT_LIMIT { lo } else { index.wrapping_add(1) }
    }
});
