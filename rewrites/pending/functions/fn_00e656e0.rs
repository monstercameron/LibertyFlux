// original: 0x00e656e0 VEHICLES_WHEEL_LOOPS_MAIN_TARMAC_SKID
/// Resolve the audio event `VEHICLES_WHEEL_LOOPS_MAIN_TARMAC_SKID` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e656e0() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8FC04), 0);
        *global::<u32>(0x01283518) = r0;
        r0
    }
});
