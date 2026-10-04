// original: 0x00E5F520 registrar_call_e6f400
// Forward a fixed table address to the registrar, return its answer.
//
// Pushes the constant address and calls the registrar with the
// cdecl convention (the caller pops the argument).
lf_checker_rt::export!(cdecl, rw_00e5f520() -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(0x00E6F400))
});
