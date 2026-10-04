// original: 0x009B7010 NativeImpl_GET_CINEMATIC_CAM
/// Return the cinematic camera's script handle, or zero when there is none.
///
/// Reads the camera object for the cinematic slot from the camera manager,
/// converts it to a script handle through the handle manager, stores the
/// handle at `out` and returns it. A null camera yields zero with no store
/// and no second call. stdcall, one out-pointer argument.
lf_checker_rt::export!(stdcall, rw_009B7010(out: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x0103E498;
        const HANDLE_MGR_SLOT: u32 = 0x012FB1A0;
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAM_MGR));
        if cam == 0 {
            return 0;
        }
        let mgr = (lf_checker_rt::global::<u32>(HANDLE_MGR_SLOT) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_thiscall!(2, u32, mgr, cam);
        (out as *mut u32).write_unaligned(h);
        h
    }
});
