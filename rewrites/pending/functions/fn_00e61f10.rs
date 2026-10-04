// original: 0x00e61f10 timer_bare_register
// Bare registration stub: hands one descriptor to the shared registrar and
// returns its answer. Takes no arguments and touches no memory.
lf_checker_rt::export!(cdecl, rw_00e61f10() -> u32 {
    const DESC: u32 = 0x00E70800;
    lf_checker_rt::callee_cdecl!(1, u32, lf_checker_rt::relocated(DESC))
});
