// original: 0x00A4C990 vehicle_probe_buffers (proposed)

/// Probes one scratch buffer through a first callee and two more through a
/// second, then returns 1.
///
/// Calls the probing callee with a frame buffer in `ecx` and no stack words,
/// then the filling callee twice (cdecl, `(buffer, 4)` and `(buffer, 0x70)`).
/// The original leaves its entry `ecx` on the stack (restored by the frame),
/// which the rewrite does not replicate: only the calls, their stack words
/// and the result are observable. Buffer addresses differ legitimately
/// between sides, so the contract skips them and snapshots the pointed-to
/// words (zeros on both sides). Always returns 1.
///
/// Original: 0x00A4C990 (thiscall, no stack words), two callees, frame args.
lf_checker_rt::export!(thiscall, rw_00A4C990(_this: u32) -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const FILL_CALLEE: u32 = 2;
        const FILL_A_TAG: u32 = 4;
        const FILL_B_TAG: u32 = 0x70;
        let mut buf_a = [0u32; 4];
        let mut buf_b = [0u32; 4];
        let mut buf_c = [0u32; 4];
        lf_checker_rt::callee_thiscall!(
            PROBE_CALLEE,
            u32,
            buf_a.as_mut_ptr() as u32
        );
        lf_checker_rt::callee_cdecl!(
            FILL_CALLEE,
            u32,
            buf_b.as_mut_ptr() as u32,
            FILL_A_TAG
        );
        lf_checker_rt::callee_cdecl!(
            FILL_CALLEE,
            u32,
            buf_c.as_mut_ptr() as u32,
            FILL_B_TAG
        );
        1
    }
});
