// original: 0x00a6edc0 CTaskComplexPlayerGun::vf1

/// Clone hook of the player-gun task: hands the live manager to the shared
/// gun-clone routine.
///
/// Fetches the current manager through the anchor at `MANAGER_ANCHOR`
/// (callee 1); a null manager yields null. Otherwise the shared routine
/// (callee 2, reached by a tail jump in the original) runs with the manager
/// in ecx and its result is returned. The tail jump becomes a plain call
/// here: the checker's tail patching keeps the comparison exact.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a6edc0(_this: u32) -> u32 {
    unsafe {
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const GET_MANAGER: u32 = 1;
        const SHARED_CLONE: u32 = 2;

        let anchor = (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(SHARED_CLONE, u32, mgr)
    }
});
