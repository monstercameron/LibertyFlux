// original: 0x00e65de0 BUMP
/// Registers the ambient-speech event `BUMP`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65de0() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90DB0), 0u32);
        *global::<u32>(0x0128442C) = id;
        id
    }
});
