// original: 0x00e62500 net_init_and_register_2500
/// Run the shared network pre-initializer, then register this unit's handler.
///
/// Calls the common no-argument setup routine, then pushes this unit's
/// handler address to the shared registrar and returns its result.
lf_checker_rt::export!(cdecl, rw_00e62500() -> u32 {
    let _setup: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    let handler = lf_checker_rt::relocated(0x00E70D50);
    lf_checker_rt::callee_cdecl!(2, u32, handler)
});
