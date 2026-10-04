// original: 0x00e5c710 register_handler
/// Register this unit's handler with the registrar.
/// Returns the registrar's answer.
export!(cdecl, rw_00e5c710() -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32, relocated(0x00E6E3A0))
});
