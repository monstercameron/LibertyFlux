// original: 0x00e5c640 init_then_register_handler
/// Run the one-time initializer, then register this unit's handler
/// with the registrar. Returns the registrar's answer.
export!(cdecl, rw_00e5c640() -> u32 {
    lf_checker_rt::callee_cdecl!(1, u32,);
    lf_checker_rt::callee_cdecl!(2, u32, relocated(0x00E6E370))
});
