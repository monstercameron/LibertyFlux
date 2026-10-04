// original: 0xe6b200 notify_sentinel_04 (proposed)

/// Notify initialisation done: call the shared notifier once
/// with this unit's fixed address argument.
///
/// Pushes one constant address (cdecl, one stack word, caller
/// pops) and returns nothing meaningful. The callee is
/// intercepted and scripted by the checker on both sides.
lf_checker_rt::export!(cdecl, rw_00e6b200() -> u32 {
    const ARG_ADDR: u32 = 0x00e72ba0;
    let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(ARG_ADDR));
    0
});
