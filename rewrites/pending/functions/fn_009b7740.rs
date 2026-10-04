// original: 0x009B7740 NativeImpl_HINT_CAM
/// Run the hint-camera command with five parameters.
///
/// Forwards the five arguments to the hint-camera selector, then runs the
/// apply step on the selected object and returns its result.
/// stdcall, five integers.
lf_checker_rt::export!(stdcall, rw_009B7740(a0: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x0103E498;
        let sel: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAM_MGR), a0, a1, a2, a3, a4);
        lf_checker_rt::callee_thiscall!(2, u32, sel)
    }
});
