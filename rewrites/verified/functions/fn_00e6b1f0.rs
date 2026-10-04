// original: 0x00e6b1f0 notify_sentinel_03 (proposed)

/// Notify initialisation done: call the shared notifier once
/// with this unit's fixed address argument.
///
/// Pushes one constant address (cdecl, one stack word, caller
/// pops) and returns nothing meaningful. The callee is
/// intercepted and scripted by the checker on both sides.
lf_checker_rt::export!(cdecl, rw_00e6b1f0() -> u32 {
    const ARG_ADDR: u32 = 0x00e72b60;
    let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ARG_ADDR));
    0
});
