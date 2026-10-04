// original: 0x00e5eba0 register_sn_event_removed_from_session
/// Build the event descriptor, then register its handler entry.
///
/// Calls the event-descriptor constructor for its side effects (the returned
/// value is discarded) and forwards the handler entry address to the
/// registrar, returning the registrar's answer.
export!(cdecl, rw_00e5eba0() -> u32 {
    let _: u32 = callee_cdecl!(1, u32,);
    callee_cdecl!(2, u32, relocated(0x00E6F210))
});
