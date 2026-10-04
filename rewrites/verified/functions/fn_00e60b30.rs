// original: 0x00E60B30 timer_obj_init_b (proposed)

/// Object initialisation plus callback registration: runs an object
/// initialiser with the object address in ECX, then registers a callback.
///
/// Behaviour: calls callee 1 (thiscall, `this` = `OBJECT`, no stack
/// arguments, return ignored), then calls callee 2 (cdecl registrar) with
/// the callback address `CALLBACK`, and returns the registrar's answer.
/// Call order is fixed; no memory is read or written besides the calls.
///
/// Original: no stack arguments; the object address is an immediate moved
/// into ECX. Return value is the registrar's answer.
lf_checker_rt::export!(cdecl, rw_00e60b30() -> u32 {
    unsafe {
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(0x0019F7F88));
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E6FC00))
    }
});

