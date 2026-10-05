// original: 0x00E66E60 task_row_then_forward_00E66E60
/// Initialise one task row, then forward a code reference.
///
/// Calls callee 1 (thiscall, no stack arguments) with `OBJ` in ECX,
/// then calls callee 2 (cdecl, one argument: the file address `ARG`,
/// relocated), discarding both results.
///
/// Original: 0x00E66E60 (cdecl, no arguments, no return value).
lf_checker_rt::export!(cdecl, rw_00E66E60() -> u32 {
    unsafe {
        const OBJ: u32 = 0x012B9170;
        const ARG: u32 = 0x00E72150;
        lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(OBJ));
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(ARG));
        0
    }
});
