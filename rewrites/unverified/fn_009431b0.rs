// original: 0x009431b0 streaming_notify_two (proposed)

/// Notify the streaming registry and the follow-up worker about an event.
///
/// Calls the registry handler (thiscall on the fixed registry object, one
/// stack argument: the event) and then the follow-up worker (cdecl, one
/// stack argument). Returns the follow-up worker's answer.
///
/// Note: the original passes its own incoming return address as the
/// follow-up worker's argument (it re-pushes the stack slot above its own
/// argument). A Rust rewrite cannot observe its return address, so the
/// contract skips that argument; everything else is compared exactly.
///
/// Original: 0x009431b0 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_009431b0(event: u32) -> u32 {
    const REGISTRY: u32 = 0x0117374C;
    const NOTIFY: u32 = 1;
    const FOLLOW_UP: u32 = 2;
    let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, REGISTRY, event);
    lf_checker_rt::callee_cdecl!(FOLLOW_UP, u32, 0u32)
});
