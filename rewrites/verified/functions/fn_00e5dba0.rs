// original: 0x00e5dba0 net_obj_init_register_dba0
/// Initialise one fixed object, then register one fixed handler address.
///
/// Runs the zero-argument initialiser on the fixed global instance, then
/// calls the central registrar with its constant address argument and
/// returns the registrar answer. Takes no inputs.
lf_checker_rt::export!(cdecl, rw_00e5dba0() -> u32 {
    lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(0x019D2DC0));
    lf_checker_rt::callee_cdecl!(3, u32, lf_checker_rt::relocated(0x00E6EC30))
});
