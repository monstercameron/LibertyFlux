// original: 0x00A82C80 CComplexGunTaskInfo::get_status_name

/// Forwards the single 32-bit stack argument to a cdecl helper and returns
/// its 32-bit result. The receiver is unused; the thiscall entry pops its
/// one stack argument while the helper leaves cleanup to this wrapper.
lf_checker_rt::export!(thiscall, rw_00A82C80(_receiver: u32, status: u32) -> u32 {
    const STATUS_NAME_HELPER: u32 = 1;
    lf_checker_rt::callee_cdecl!(STATUS_NAME_HELPER, u32, status)
});
