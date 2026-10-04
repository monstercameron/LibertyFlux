// original: 0x009B73B0 NativeImpl_GET_GAME_CAM
/// Return the game camera's script handle.
///
/// Same shape as the free-camera getter with the game-camera selector.
/// stdcall, one out-pointer argument.
lf_checker_rt::export!(stdcall, rw_009B73B0(out: u32) -> u32 {
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
