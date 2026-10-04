// original: 0x00e625e0 net_register_handler_25e0
/// Register one network callback with the shared registrar.
///
/// Pushes this unit's handler address and forwards it to the common
/// registration routine, returning that routine's result.
lf_checker_rt::export!(cdecl, rw_00e625e0() -> u32 {
    let handler = lf_checker_rt::relocated(0x00E70F10);
    lf_checker_rt::callee_cdecl!(1, u32, handler)
});
