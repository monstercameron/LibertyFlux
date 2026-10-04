// original: 0x00e65e40 COP_HELI_MEGAPHONE_WARNING
/// Registers the ambient-speech event `COP_HELI_MEGAPHONE_WARNING`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65e40() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90EEC), 0u32);
        *global::<u32>(0x01284594) = id;
        id
    }
});
