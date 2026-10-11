// original: 0x00c768d0 CPedModelInfo::vf9

/// Forward the one stack argument to the resource setter, refresh the model,
/// and then refresh its dependent animation data. Return the final refresh
/// call's EAX value; the thiscall callee removes the incoming stack argument.
lf_checker_rt::export!(thiscall, rw_00c768d0(this: u32, value: u32) -> u32 {
    const SET_RESOURCE: u32 = 1;
    const REFRESH_MODEL: u32 = 2;
    const REFRESH_ANIMATIONS: u32 = 3;
    let _ = lf_checker_rt::callee_thiscall!(SET_RESOURCE, u32, this, value);
    let _ = lf_checker_rt::callee_thiscall!(REFRESH_MODEL, u32, this);
    lf_checker_rt::callee_thiscall!(REFRESH_ANIMATIONS, u32, this)
});
