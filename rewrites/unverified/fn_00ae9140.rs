// original: 0x00AE9140 refresh_coarse_timer

/// Replaces the coarse timer global with the result of the shared timer
/// sampler. The sampler has no explicit arguments.
///
/// Calling convention: cdecl with no arguments and no return value.
lf_checker_rt::export!(cdecl, rw_00ae9140() -> () {
    const COARSE_TIMER_VA: u32 = 0x0159_3bc8;
    const TIMER_SAMPLE_CALLEE: u32 = 1;
    let sampled_time = lf_checker_rt::callee_cdecl!(TIMER_SAMPLE_CALLEE, u32,);
    unsafe {
        lf_checker_rt::global::<u32>(COARSE_TIMER_VA).write_unaligned(sampled_time);
    }
});
