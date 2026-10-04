// original: 0x00e65f00 INTIMIDATE_RESP
/// Registers the ambient-speech event `INTIMIDATE_RESP`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65f00() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90DD4), 0u32);
        *global::<u32>(0x0128443C) = id;
        id
    }
});
