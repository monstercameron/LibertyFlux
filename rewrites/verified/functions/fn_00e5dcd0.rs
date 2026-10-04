// original: 0x00e5dcd0 net_reset_and_register_dcd0
/// Reset shared state, then register one fixed handler address.
///
/// Runs the zero-argument reset routine, then calls the central
/// registrar with its constant address argument and returns the
/// registrar answer. Takes no inputs.
lf_checker_rt::export!(cdecl, rw_00e5dcd0() -> u32 {
    lf_checker_rt::callee_cdecl!(2, u32,);
    lf_checker_rt::callee_cdecl!(3, u32, lf_checker_rt::relocated(0x00E6ECC0))
});
