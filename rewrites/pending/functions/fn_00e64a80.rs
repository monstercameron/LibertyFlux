// original: 0x00e64a80 SHELL_CASINGS_PLASTIC_BOUNCE
/// Resolve the audio event `SHELL_CASINGS_PLASTIC_BOUNCE` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e64a80() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8E700), 0);
        *global::<u32>(0x01283200) = r0;
        r0
    }
});
