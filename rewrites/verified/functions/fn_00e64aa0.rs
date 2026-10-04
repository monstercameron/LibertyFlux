// original: 0x00e64aa0 SHOTGUN_ECHO
/// Resolve the audio event `SHOTGUN_ECHO` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e64aa0() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8E6F0), 0);
        *global::<u32>(0x012831FC) = r0;
        r0
    }
});
