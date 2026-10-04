// original: 0x00e65a40 FUCK_FALL
/// Resolve the audio event `FUCK_FALL` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e65a40() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E90C38), 0);
        *global::<u32>(0x01284564) = r0;
        r0
    }
});
