// original: 0x00E688C0 veh_register_callback (proposed)
/// Register one static vehicle callback block with the registrar.
///
/// Passes `ARG`, the address of a static code block, to the registrar callee
/// (`CALLEE`, cdecl, one argument) and discards the result. No memory is
/// touched directly.
///
/// Original: 0x00E688C0 (cdecl, no arguments, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00e688c0() -> u32 {
    unsafe {
        const ARG: u32 = 0x00E72480;
        const CALLEE: u32 = 1;
        lf_checker_rt::callee_cdecl!(CALLEE, u32,
            lf_checker_rt::relocated(ARG));
        0
    }
});
