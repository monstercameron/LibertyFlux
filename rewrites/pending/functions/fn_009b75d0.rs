// original: 0x009B75D0 NativeImpl_GET_SCRIPT_DRAW_CAM
/// Return the script draw camera's script handle via its wrapper object.
///
/// Two-hop shape like the debug-camera getter. stdcall, one out-pointer.
lf_checker_rt::export!(stdcall, rw_009B75D0(out: u32) -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x0103E498;
        const HANDLE_MGR_SLOT: u32 = 0x012FB1A0;
        let cam: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAM_MGR));
        let inner: u32 = lf_checker_rt::callee_thiscall!(2, u32, cam);
        let mgr = (lf_checker_rt::global::<u32>(HANDLE_MGR_SLOT) as *const u32).read_unaligned();
        let h: u32 = lf_checker_rt::callee_thiscall!(3, u32, mgr, inner);
        (out as *mut u32).write_unaligned(h);
        h
    }
});
