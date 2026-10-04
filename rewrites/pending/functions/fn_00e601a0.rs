// original: 0x00e601a0 timer_slot_init_02
// Per-slot timer setup stub: runs this slot's setup step, then hands the
// slot descriptor to the shared registrar and returns the registrar's
// answer. Takes no arguments and touches no memory of its own.
lf_checker_rt::export!(cdecl, rw_00e601a0() -> u32 {
    const DESC: u32 = 0x00E6F740;
    let _setup: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});
