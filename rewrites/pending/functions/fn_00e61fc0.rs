// original: 0x00e61fc0 timer_reg_slot_04
// Per-slot timer registration stub: runs this slot's setup step, then hands
// the slot descriptor to the shared registrar and returns the registrar's
// answer. Takes no arguments and touches no memory of its own.
lf_checker_rt::export!(cdecl, rw_00e61fc0() -> u32 {
    const DESC: u32 = 0x00E70870;
    let _setup: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});
