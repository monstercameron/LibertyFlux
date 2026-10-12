// original: 0x008B5700 frontend_invalidate_all_handles

/// Release and invalidate all six frontend handle slots.
///
/// Calls the slot invalidator once per slot index 0 through 5 and returns
/// the last call's result. Cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_008B5700() -> u32 {
    unsafe {
        const SLOT_COUNT: u32 = 6;
        let mut last = 0u32;
        let mut slot = 0u32;
        while slot < SLOT_COUNT {
            last = lf_checker_rt::callee_cdecl!(1, u32, slot);
            slot += 1;
        }
        last
    }
});
