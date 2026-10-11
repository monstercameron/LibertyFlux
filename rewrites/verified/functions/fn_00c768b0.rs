// original: 0x00c768b0 CPedModelInfo::vf6

/// Forward the single stack argument to the object's resource setter, then
/// refresh its derived model state. Return the refresh call's EAX value; the
/// thiscall callee removes the one incoming stack argument.
lf_checker_rt::export!(thiscall, rw_00c768b0(this: u32, value: u32) -> u32 {
    const SET_RESOURCE: u32 = 1;
    const REFRESH_MODEL: u32 = 2;
    let _ = lf_checker_rt::callee_thiscall!(SET_RESOURCE, u32, this, value);
    lf_checker_rt::callee_thiscall!(REFRESH_MODEL, u32, this)
});
