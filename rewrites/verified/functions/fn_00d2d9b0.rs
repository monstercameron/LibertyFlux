// original: 0x00d2d9b0 task_forward_11a (proposed)
/// Forward to the shared task routine with tag 0x11a and the caller's
/// argument, returning its result. `this` passes through in ecx.
///
/// Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00d2d9b0(this: u32, arg: u32) -> u32 {
    const TAG: u32 = 0x11a;
    const ROUTINE: u32 = 1;
    lf_checker_rt::callee_thiscall!(ROUTINE, u32, this, TAG, arg)
});
