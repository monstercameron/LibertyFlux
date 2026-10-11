// original: 0x00A82CA0 CComplexNewGetInVehicleTaskInfo::get_status_name

/// Return the status label supplied by the helper for the forwarded status value. The object pointer is unused; the value is passed as one cdecl word.
lf_checker_rt::export!(thiscall, rw_00a82ca0(this: u32, status: u32) -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32, status)
});
