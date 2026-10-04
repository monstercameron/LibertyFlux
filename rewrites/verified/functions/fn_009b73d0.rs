// original: 0x009B73D0 NativeImpl_GET_GAME_CAM_CHILD
/// Return the game camera child's script handle, trying two selectors.
///
/// Queries the camera child with selector 1 and, when that yields nothing,
/// retries with selector 2. A double miss returns zero; otherwise the result
/// is converted to a script handle and returned. stdcall, no arguments.
lf_checker_rt::export!(stdcall, rw_009B73D0() -> u32 {
    unsafe {
        const CAM_MGR: u32 = 0x0103E498;
        const HANDLE_MGR_SLOT: u32 = 0x012FB1A0;
        let m: u32 = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAM_MGR));
        let mut r: u32 = lf_checker_rt::callee_thiscall!(2, u32, m, 1, 0);
        if r == 0 {
            r = lf_checker_rt::callee_thiscall!(2, u32, m, 2, 0);
            if r == 0 {
                return 0;
            }
        }
        let mgr = (lf_checker_rt::global::<u32>(HANDLE_MGR_SLOT) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(3, u32, mgr, r)
    }
});
