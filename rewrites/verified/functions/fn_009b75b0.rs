// original: 0x009B75B0 NativeImpl_GET_SCRIPT_CAM
/// Return the script camera's script handle.
///
/// Same shape as the other single-hop camera getters. stdcall, one out-pointer.
lf_checker_rt::export!(stdcall, rw_009B75B0(out: u32) -> u32 {
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
