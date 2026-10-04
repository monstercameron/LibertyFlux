// original: 0x00e619b0 timing_unit_init_4
/// Initialise one timer unit, then register its handler.
///
/// Runs the unit initialiser (stubbed, no arguments; its answer is ignored), then passes the code pointer `0x00E70430` to the registrar helper (stubbed, cdecl/1) and returns its answer.
export!(cdecl, rw_00e619b0() -> u32 {
    unsafe {
        let _: u32 = callee_cdecl!(0, u32,);
        callee_cdecl!(1, u32, relocated(0x00e70430))
    }
});
