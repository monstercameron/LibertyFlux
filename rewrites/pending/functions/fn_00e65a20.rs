// original: 0x00e65a20 DROWNING
/// Resolve the audio event `DROWNING` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e65a20() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E90C04), 0);
        *global::<u32>(0x01284404) = r0;
        r0
    }
});
