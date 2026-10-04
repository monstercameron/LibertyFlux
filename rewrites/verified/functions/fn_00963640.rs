// original: 0x00963640 replay_mode_select (proposed)
//
// Checked-in text of this file is in out/rewrites/fn_00963640.rs; this crate
// file also carries the mutant (in-crate only, never shipped).

/// Select a replay/file mode and publish its numeric parameter.
///
/// `mode` is the requested mode; `caller_word` is the stack word above the
/// argument, which the original reads on two paths (its caller's data).
///
/// Modes 0 and 3..=6 store the mode id at `MODE` and a fixed parameter at
/// `PARAM` (0->0, 3->15, 4->50, 5->75, 6->99) and return the mode. Mode 1
/// stores 1, calls the two helpers (the second with `helper_a() -
/// caller_word`), and derives the parameter from the fourth dword (offset 12)
/// of the 32-byte block the second helper returns:
/// `PARAM = BASE_B - (block[3] - BASE_A)`. Mode 2 stores 2 with
/// `PARAM = BASE_B - caller_word`. Modes 1 and 2 also store `PARAM` converted
/// to float at `PARAM_F` and return the sign bit of the parameter (0 or 1).
/// Any other mode writes nothing and returns the mode.
///
/// The 32-byte block is copied through the original's own stack frame, which
/// the checker does not observe (below-ESP scratch); only the dword at offset
/// 12 feeds the result, so only that dword is read here.
///
/// Original: 0x00963640 (cdecl, one stack word plus the caller word above it).
lf_checker_rt::export!(cdecl, rw_00963640(mode: u32, caller_word: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x011F70DC;
        const PARAM: u32 = 0x011F70D8;
        const BASE_A: u32 = 0x011F7028;
        const BASE_B: u32 = 0x011F7030;
        const PARAM_F: u32 = 0x011F7054;
        const HELPER_A: u32 = 1;
        const HELPER_B: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        /// Shared tail of modes 1 and 2: store the parameter, its float
        /// conversion, and return the parameter's sign bit.
        #[inline(always)]
        unsafe fn tail(param: u32) -> u32 {
            unsafe {
                wr32(lf_checker_rt::relocated(PARAM), param);
                wr32(
                    lf_checker_rt::relocated(PARAM_F),
                    (param as f32).to_bits(),
                );
                param >> 31
            }
        }

        match mode {
            0 => {
                wr32(lf_checker_rt::relocated(MODE), 0);
                wr32(lf_checker_rt::relocated(PARAM), 0);
                0
            }
            3 => {
                wr32(lf_checker_rt::relocated(MODE), 3);
                wr32(lf_checker_rt::relocated(PARAM), 15);
                3
            }
            4 => {
                wr32(lf_checker_rt::relocated(MODE), 4);
                wr32(lf_checker_rt::relocated(PARAM), 50);
                4
            }
            5 => {
                wr32(lf_checker_rt::relocated(MODE), 5);
                wr32(lf_checker_rt::relocated(PARAM), 75);
                5
            }
            6 => {
                wr32(lf_checker_rt::relocated(MODE), 6);
                wr32(lf_checker_rt::relocated(PARAM), 99);
                6
            }
            1 => {
                wr32(lf_checker_rt::relocated(MODE), 1);
                let t: u32 = lf_checker_rt::callee_cdecl!(HELPER_A, u32,);
                let block: u32 =
                    lf_checker_rt::callee_cdecl!(HELPER_B, u32, t.wrapping_sub(caller_word));
                let word4 = rd32(block.wrapping_add(12));
                let param = rd32(lf_checker_rt::relocated(BASE_B))
                    .wrapping_sub(word4.wrapping_sub(rd32(lf_checker_rt::relocated(BASE_A))));
                tail(param)
            }
            2 => {
                let param = rd32(lf_checker_rt::relocated(BASE_B)).wrapping_sub(caller_word);
                wr32(lf_checker_rt::relocated(MODE), 2);
                tail(param)
            }
            _ => mode,
        }
    }
});
