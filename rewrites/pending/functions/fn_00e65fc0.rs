// original: 0x00e65fc0 KILLED_ALL
/// Registers the ambient-speech event `KILLED_ALL`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65fc0() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90E24), 0u32);
        *global::<u32>(0x01284524) = id;
        id
    }
});
