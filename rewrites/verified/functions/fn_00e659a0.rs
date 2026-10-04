// original: 0x00e659a0 BRIAN_CLEAN
/// Resolve the audio event `BRIAN_CLEAN` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e659a0() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E90D20), 0);
        *global::<u32>(0x01284568) = r0;
        r0
    }
});
