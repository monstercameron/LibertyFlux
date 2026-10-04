// original: 0x00E66320 construct_state_01288780_and_register_routine (proposed)

/// Construct one global state object and register its handler routine.
///
/// Calls the object constructor (thiscall, object address in ecx, no stack
/// arguments) with the global object, then passes the handler routine's
/// code address to the registrar callee (cdecl, one argument; the caller
/// drops it with a pop). No arguments; returns the registrar's answer
/// (cdecl).
lf_checker_rt::export!(cdecl, rw_00e66320() -> u32 {
    const OBJECT: u32 = 0x01288780;
    const ROUTINE: u32 = 0x00E71EC0;
    const CTOR_CALLEE: u32 = 1;
    const REGISTRAR_CALLEE: u32 = 2;
    let obj = lf_checker_rt::relocated(OBJECT);
    let _: u32 = lf_checker_rt::callee_thiscall!(CTOR_CALLEE, u32, obj);
    lf_checker_rt::callee_cdecl!(REGISTRAR_CALLEE, u32, lf_checker_rt::relocated(ROUTINE))
});
