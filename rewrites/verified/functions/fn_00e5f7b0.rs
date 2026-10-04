// original: 0x00E5F7B0 init_and_register_e6f420
// Run a zero-argument initializer, then register a fixed address.
//
// Calls the initializer, discards its answer, then forwards the
// constant table address to the registrar (cdecl) and returns
// that answer.
lf_checker_rt::export!(cdecl, rw_00e5f7b0() -> u32 {
    let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E6F420))
});
