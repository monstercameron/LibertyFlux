// original: 0x00e62060 timer_reg_slot_09
// Per-slot timer registration stub: runs this slot's setup step, then hands
// the slot descriptor to the shared registrar and returns the registrar's
// answer. Takes no arguments and touches no memory of its own.
lf_checker_rt::export!(cdecl, rw_00e62060() -> u32 {
    const DESC: u32 = 0x00E708C0;
    let _setup: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});
