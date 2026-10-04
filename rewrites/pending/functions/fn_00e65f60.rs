// original: 0x00e65f60 JACKED_IN_CAR
/// Registers the ambient-speech event `JACKED_IN_CAR`.
///
/// Resolves the event name through the engine lookup helper (called with the
/// name pointer and a zero flag), stores the returned id in this event's
/// dedicated global slot, and returns the id.
export!(cdecl, rw_00e65f60() -> u32 {
    unsafe {
        let id = callee_cdecl!(1, u32, relocated(0x00E90E54), 0u32);
        *global::<u32>(0x01284520) = id;
        id
    }
});
