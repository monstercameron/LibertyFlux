// original: 0x00E67240 init_single_and_register

/// Initialise one fixed object, then register a fixed stub.
///
/// Calls the first callee with the object pointer `OBJ` in ECX (no stack
/// arguments), then calls the registrar callee (cdecl, one argument) with the
/// constant stub address `STUB`, and returns the registrar's answer. Both
/// addresses are loader-relocated in the original and derived from the
/// relocated image base here.
///
/// Original: 0x00E67240 (cdecl, no arguments, two outgoing calls, returns last result).
lf_checker_rt::export!(cdecl, rw_00e67240() -> u32 {
    const OBJ: u32 = 0x012DDEFC;
    const STUB: u32 = 0x00E721B0;
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJ));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(STUB))
});
