// original: 0x00E667B0 construct_state_0128e94c_and_register_routine (proposed)

/// Construct one global state object and register its handler routine.
///
/// Calls the object constructor (thiscall, object address in ecx, no stack
/// arguments) with the global object, then passes the handler routine's
/// code address to the registrar callee (cdecl, one argument; the caller
/// drops it with a pop). No arguments; returns the registrar's answer
/// (cdecl).
lf_checker_rt::export!(cdecl, rw_00e667b0() -> u32 {
    const OBJECT: u32 = 0x0128E94C;
    const ROUTINE: u32 = 0x00E71F90;
    const CTOR_CALLEE: u32 = 1;
    const REGISTRAR_CALLEE: u32 = 2;
    let obj = lf_checker_rt::relocated(OBJECT);
    let _: u32 = lf_checker_rt::callee_thiscall!(CTOR_CALLEE, u32, obj);
    lf_checker_rt::callee_cdecl!(REGISTRAR_CALLEE, u32, lf_checker_rt::relocated(ROUTINE))
});
