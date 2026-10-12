// original: 0x00c61580 CTaskSimpleNewGangDriveBy::process_attack

/// Forwards the explicit argument to the task's weapon firing method, then
/// returns false in `AL`. The original does not inspect either the argument
/// or the callee's result. This thiscall method has one stack argument.
lf_checker_rt::export!(thiscall, rw_00c61580(this: u32, forwarded_arg: u32) -> u8 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, forwarded_arg);
    0
});
