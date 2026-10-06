// original: 0x008BF750 FRONTEND_MENU_SELECT (symbols)

/// Handle a frontend menu selection: store the menu id, open the menu's data
/// and jump to the shared menu runner.
///
/// When the frontend-active byte is nonzero the function does nothing and
/// returns the incoming eax untouched (the contract pins it to zero).
/// Otherwise the menu id (1, or 2 when the alternate-menu byte is nonzero)
/// is stored through the mode-setter callee, the open callee runs as a
/// thiscall on the fixed menu object with the menu's data address (one of two
/// constants, matching the id), and control passes to the menu-runner callee
/// by tail jump, whose answer is returned (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_008BF750() -> u32 {
    unsafe {
        /// Frontend-active byte: nonzero means do nothing.
        const FRONTEND_ACTIVE: u32 = 0x01037868;
        /// Alternate-menu byte: nonzero selects menu id 2.
        const ALT_MENU: u32 = 0x011609F6;
        /// Fixed menu object for the open call.
        const MENU_OBJECT: u32 = 0x01176888;
        /// Menu data addresses for id 1 and id 2.
        const MENU_DATA_1: u32 = 0x00E7E010;
        const MENU_DATA_2: u32 = 0x00E7E028;
        /// Callee ids in the contract: mode setter, open, tail runner.
        const SET_MODE: u32 = 1;
        const OPEN: u32 = 2;
        const RUNNER: u32 = 3;
        if (lf_checker_rt::relocated(FRONTEND_ACTIVE) as *const u8).read() != 0 {
            // Incoming eax passes through; the contract pins it to zero.
            return 0;
        }
        let alt = (lf_checker_rt::relocated(ALT_MENU) as *const u8).read() != 0;
        let id = if alt { 2u32 } else { 1u32 };
        lf_checker_rt::callee_cdecl!(SET_MODE, u32, id);
        let data = if alt { MENU_DATA_2 } else { MENU_DATA_1 };
        lf_checker_rt::callee_thiscall!(
            OPEN,
            u32,
            lf_checker_rt::relocated(MENU_OBJECT),
            lf_checker_rt::relocated(data)
        );
        lf_checker_rt::callee_cdecl!(RUNNER, u32,)
    }
});
