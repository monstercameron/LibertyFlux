// original: 0x00e65dc0 BEEN_SHOT
/// Registers the ambient-speech event `BEEN_SHOT`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65dc0() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90EC0), 0u32);
        *global::<u32>(0x01284534) = id;
        id
    }
});
