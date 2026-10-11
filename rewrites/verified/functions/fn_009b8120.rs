// original: 0x009B8120 CamApplyState
/// Forwards its two stack arguments, in order, to the camera manager at its relocated global address. It passes the returned object in ECX to a second helper and returns that helper's EAX result.
lf_checker_rt::export!(stdcall, rw_009b8120(arg0: u32, arg1: u32) -> u32 {
    const CAMERA_MANAGER_VA: u32 = 0x0128E400;
    let manager = lf_checker_rt::relocated(CAMERA_MANAGER_VA);
    let state = lf_checker_rt::callee_thiscall!(1, u32, manager, arg0, arg1);
    lf_checker_rt::callee_thiscall!(2, u32, state)
});
