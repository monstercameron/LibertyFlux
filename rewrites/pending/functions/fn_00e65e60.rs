// original: 0x00e65e60 DIRECTIONS_NO
/// Registers the ambient-speech event `DIRECTIONS_NO`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65e60() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90E94), 0u32);
        *global::<u32>(0x01284450) = id;
        id
    }
});
