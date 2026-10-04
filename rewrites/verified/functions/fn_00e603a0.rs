// original: 0x00e603a0 timer_slot_init_17
// Per-slot timer setup stub: runs this slot's setup step, then hands the
// slot descriptor to the shared registrar and returns the registrar's
// answer. Takes no arguments and touches no memory of its own.
lf_checker_rt::export!(cdecl, rw_00e603a0() -> u32 {
    const DESC: u32 = 0x00E6F870;
    let _setup: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});

// mutant: drops the setup call; must fail on the call comparison.
lf_checker_rt::export!(cdecl, mut_00e60160() -> u32 {
    const DESC: u32 = 0x00E6F720;
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});
