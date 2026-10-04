// original: 0x00e65d80 SPEECH_AMBIENT_POSITIONED
/// Registers the ambient-speech event `SPEECH_AMBIENT_POSITIONED`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65d80() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90D5C), 0u32);
        *global::<u32>(0x012844E4) = id;
        id
    }
});
