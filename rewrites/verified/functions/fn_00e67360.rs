// original: 0x00E67360 init_single_and_register_01

/// Initialise one fixed object, then register a fixed stub.
///
/// Calls the first callee with the object pointer `OBJ` in ECX (no stack
/// arguments), then calls the registrar callee (cdecl, one argument) with the
/// constant stub address `STUB`, and returns the registrar's answer. Both
/// addresses are loader-relocated in the original and derived from the
/// relocated image base here.
///
/// Original: 0x00E67360 (cdecl, no arguments, two outgoing calls, returns last result).
lf_checker_rt::export!(cdecl, rw_00e67360() -> u32 {
    const OBJ: u32 = 0x012DFD80;
    const STUB: u32 = 0x00E721F0;
    lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJ));
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(STUB))
});
