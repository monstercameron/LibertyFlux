// original: 0x00e656c0 IS_FUCKED
/// Resolve the audio event `isFucked` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e656c0() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8FCD4), 0);
        *global::<u32>(0x01284164) = r0;
        r0
    }
});
