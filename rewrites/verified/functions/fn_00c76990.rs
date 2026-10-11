// original: 0x00c76990 CPedModelInfo::vf4

/// Forward the two stack arguments to the object's resource selector, then
/// refresh dependent animation data. Return the refresh call's EAX value; the
/// thiscall callee removes both incoming stack arguments.
lf_checker_rt::export!(thiscall, rw_00c76990(this: u32, key: u32, value: u32) -> u32 {
    const SELECT_RESOURCE: u32 = 1;
    const REFRESH_ANIMATIONS: u32 = 2;
    let _ = lf_checker_rt::callee_thiscall!(SELECT_RESOURCE, u32, this, key, value);
    lf_checker_rt::callee_thiscall!(REFRESH_ANIMATIONS, u32, this)
});
