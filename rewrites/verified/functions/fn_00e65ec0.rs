// original: 0x00e65ec0 GANG_DODGE_WARNING
/// Registers the ambient-speech event `GANG_DODGE_WARNING`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65ec0() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90EAC), 0u32);
        *global::<u32>(0x01284598) = id;
        id
    }
});
