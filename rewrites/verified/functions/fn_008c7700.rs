// original: 0x008C7700 sg_hdg_auto
/// Automatically bring the streaming backend up or leave it idle.
///
/// A negative `mode` with the auto flag clear reports 0 at once; any
/// nonzero state or a set done flag also reports 0 or 2 without acting.
/// Otherwise the prepare callee runs, and a negative mode takes the
/// default-source path (default constant, mark, announce) while a
/// non-negative mode takes the bind path (bind, announce). Done and state
/// are recorded and the result is 0. Original: cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_008c7700(mode: u32) -> u32 {
    unsafe {
        const PREPARE_CALLEE: u32 = 1;
        const DEFAULT_CALLEE: u32 = 2;
        const ANNOUNCE_A_CALLEE: u32 = 3;
        const BIND_CALLEE: u32 = 4;
        const ANNOUNCE_B_CALLEE: u32 = 5;
        const DEFAULT_SRC_FILE_VA: u32 = 0x00E80564;
        if (mode as i32) < 0
            && *lf_checker_rt::global::<u32>(0x1160C50) == 0
        {
            return 0;
        }
        match *lf_checker_rt::global::<u32>(0x11730F8) {
            0 => {}
            _ => return 2,
        }
        if *lf_checker_rt::global::<u32>(0x1172F54) != 0 {
            return 0;
        }
        lf_checker_rt::callee_cdecl!(PREPARE_CALLEE, u32,);
        *lf_checker_rt::global::<u8>(0x1172F53) = 0;
        if (mode as i32) < 0 {
            let src = lf_checker_rt::relocated(DEFAULT_SRC_FILE_VA);
            *lf_checker_rt::global::<u8>(0x1172F53) = 1;
            lf_checker_rt::callee_cdecl!(DEFAULT_CALLEE, u32, src);
            lf_checker_rt::callee_cdecl!(ANNOUNCE_A_CALLEE, u32,);
        } else {
            lf_checker_rt::callee_cdecl!(BIND_CALLEE, u32, mode);
            lf_checker_rt::callee_cdecl!(ANNOUNCE_B_CALLEE, u32,);
        }
        *lf_checker_rt::global::<u32>(0x1172F54) = 1;
        *lf_checker_rt::global::<u32>(0x11730F8) = 1;
        0
    }
});
