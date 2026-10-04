// original: 0x00e60380 timer_slot_init_16
// Per-slot timer setup stub: runs this slot's setup step, then hands the
// slot descriptor to the shared registrar and returns the registrar's
// answer. Takes no arguments and touches no memory of its own.
lf_checker_rt::export!(cdecl, rw_00e60380() -> u32 {
    const DESC: u32 = 0x00E6F860;
    let _setup: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
    lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(DESC))
});
