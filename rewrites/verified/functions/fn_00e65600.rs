// original: 0x00e65600 VEHICLE_WHEEL_LOOPS_FAST_CONCRETE
/// Resolve the audio event `VEHICLE_WHEEL_LOOPS_FAST_CONCRETE` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e65600() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8FC7C), 0);
        *global::<u32>(0x012834C4) = r0;
        r0
    }
});
