// original: 0x00e65fe0 KNOCK_OVER_PED
/// Registers the ambient-speech event `KNOCK_OVER_PED`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65fe0() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90DEC), 0u32);
        *global::<u32>(0x0128441C) = id;
        id
    }
});
