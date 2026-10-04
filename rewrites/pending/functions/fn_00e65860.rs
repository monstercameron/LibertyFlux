// original: 0x00e65860 VEHICLES_WHEEL_LOOPS_SIDE_TARMAC_SKID
/// Resolve the audio event `VEHICLES_WHEEL_LOOPS_SIDE_TARMAC_SKID` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e65860() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8FC54), 0);
        *global::<u32>(0x01283D4C) = r0;
        r0
    }
});
