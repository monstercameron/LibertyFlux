// original: 0x00E66D90 register_stub_07

/// Register a fixed init stub with the game's deferred-call registrar.
///
/// Pushes the constant stub address `STUB` and calls the registrar callee
/// (cdecl, one argument), then discards the pushed word and returns the
/// registrar's answer. The stub address is position-independent: it is derived
/// from the relocated image base, matching the loader-relocated immediate in
/// the original.
///
/// Original: 0x00E66D90 (cdecl, no arguments, one outgoing call, returns its result).
lf_checker_rt::export!(cdecl, rw_00e66d90() -> u32 {
    const STUB: u32 = 0x00E72110;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(STUB))
});
