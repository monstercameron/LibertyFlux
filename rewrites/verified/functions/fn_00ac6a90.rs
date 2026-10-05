// original: 0x00AC6A90 stream_apply_pending_state (proposed)

/// Flag the state dirty and apply the two pending state blocks.
///
/// The original sets the dirty byte, then calls the first applier with the
/// first block address and the second applier with the second block address
/// (cdecl, no arguments). No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6A90() -> u32 {
    unsafe {
        const DIRTY: u32 = 0x0103F254;
        const BLOCK0: u32 = 0x0154E070;
        const BLOCK1: u32 = 0x0154E080;
        const APPLY0: u32 = 1;
        const APPLY1: u32 = 2;
        (lf_checker_rt::relocated(DIRTY) as *mut u8).write(1);
        lf_checker_rt::callee_cdecl!(APPLY0, u32, lf_checker_rt::relocated(BLOCK0));
        lf_checker_rt::callee_cdecl!(APPLY1, u32, lf_checker_rt::relocated(BLOCK1));
        0
    }
});
