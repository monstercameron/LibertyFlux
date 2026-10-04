// original: 0x00e649e0 QUB3D_MUSIC_POSITIONED_FOR_ARCADE
/// Resolve the audio event `QUB3D_MUSIC_POSITIONED_FOR_ARCADE` to its runtime handle and cache it.
///
/// Looks the event name up through the shared name-lookup routine
/// (called with the name pointer and a zero flag) and stores the
/// returned handle in this event's slot. Returns the handle.
export!(cdecl, rw_00e649e0() -> u32 {
    unsafe {
        let r0 = callee_cdecl!(1, u32, relocated(0x00E8E070), 0);
        *global::<u32>(0x012389DC) = r0;
        r0
    }
});
