// original: 0x00e65840 VEHICLES_WHEEL_LOOPS_TARMAC_SCRAPE
/// Resolve the audio event `VEHICLES_WHEEL_LOOPS_TARMAC_SCRAPE` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e65840() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8FAB4), 0);
        *global::<u32>(0x01283844) = r0;
        r0
    }
});
