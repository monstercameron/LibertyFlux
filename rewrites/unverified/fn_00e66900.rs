// original: 0x00E66900 register_routine_00e72010 (proposed)

/// Register one handler routine with the registrar.
///
/// Passes the routine's code address to the registrar callee (cdecl,
/// one argument; the caller drops it with a pop). No arguments; returns
/// the registrar's answer (cdecl).
lf_checker_rt::export!(cdecl, rw_00e66900() -> u32 {
    const ROUTINE: u32 = 0x00E72010;
    const REGISTRAR_CALLEE: u32 = 1;
    lf_checker_rt::callee_cdecl!(REGISTRAR_CALLEE, u32, lf_checker_rt::relocated(ROUTINE))
});
