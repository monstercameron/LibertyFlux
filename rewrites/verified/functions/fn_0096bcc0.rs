// original: 0x0096BCC0 all_timing_slots_ready

/// Returns true when all five bytes in the global timing readiness table are
/// nonzero. The scan stops at the first zero and the boolean is returned in AL.
lf_checker_rt::export!(cdecl, rw_0096bcc0() -> u32 {
    const READINESS_GLOBAL: u32 = 0x016c8d64;
    unsafe {
        for slot in 0..5usize {
            if lf_checker_rt::global::<u8>(READINESS_GLOBAL).add(slot).read() == 0 {
                return 0;
            }
        }
    }
    1
});
