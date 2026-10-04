// original: 0x009B7390 NativeImpl_GET_FREE_CAM
/// Return the free camera's script handle.
///
/// Reads the free camera from the camera manager, converts it to a script
/// handle, stores it at `out` and returns it. stdcall, one out-pointer.
lf_checker_rt::export!(stdcall, rw_009B7390(out: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x0103E498;
        const HANDLE_MGR_SLOT: u32 = 0x012FB1A0;
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAM_MGR));
        let mgr = (lf_checker_rt::global::<u32>(HANDLE_MGR_SLOT) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_thiscall!(2, u32, mgr, cam);
        (out as *mut u32).write_unaligned(h);
        h
    }
});
