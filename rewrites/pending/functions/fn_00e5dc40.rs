// original: 0x00e5dc40 net_register_dc40
/// Register one fixed handler address with the central registrar.
///
/// Calls the registrar once with its constant address argument and returns
/// the answer. Takes no inputs and touches no global state.
lf_checker_rt::export!(cdecl, rw_00e5dc40() -> u32 {
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E6EC90))
});
