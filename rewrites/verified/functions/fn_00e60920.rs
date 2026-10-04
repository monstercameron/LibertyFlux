// original: 0x00E60920 timer_init_pair_c (proposed)

/// Paired subsystem initialisation: runs one subsystem initialiser, then
/// registers a callback with the registrar.
///
/// Behaviour: calls callee 1 (the subsystem initialiser, no arguments,
/// return ignored), then calls callee 2 (the registrar) with one argument,
/// the callback address `CALLBACK`, and returns the registrar's answer.
/// The two calls happen in this order on every execution; no memory is
/// read or written besides the calls themselves.
///
/// Original: cdecl, no stack arguments, callee-cleanup balanced
/// (`push`/`call`/`pop`), return value is the registrar's answer.
lf_checker_rt::export!(cdecl, rw_00e60920() -> u32 {
    unsafe {
        lf_checker_rt::callee_cdecl!(1, u32,);
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E6FB00))
    }
});

