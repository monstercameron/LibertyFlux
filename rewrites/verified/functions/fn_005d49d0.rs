// original: 0x005d49d0 CViewportGame::vf12
//
// Forward the caller's word to the base viewport method, then ask the
// global viewport manager to update this game viewport and return its result.
lf_checker_rt::export!(thiscall, rw_005d49d0(this: u32, options: u32) -> u32 {
    unsafe {
        const BASE_UPDATE: u32 = 1;
        const MANAGER_UPDATE: u32 = 2;
        let _: u32 = lf_checker_rt::callee_thiscall!(BASE_UPDATE, u32, this, options);
        lf_checker_rt::callee_thiscall!(MANAGER_UPDATE, u32, lf_checker_rt::relocated(0x0161_547c), this)
    }
});
