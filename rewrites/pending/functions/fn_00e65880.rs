// original: 0x00e65880 TEMPERATURE
/// Resolve the audio event `temperature` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e65880() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8FCBC), 0);
        *global::<u32>(0x01283A00) = r0;
        r0
    }
});
