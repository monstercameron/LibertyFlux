// original: 0x00bbabd0 TASK_START_SCENARIO_IN_PLACE
/// Start a scenario for a character at its current position.
///
/// Forwards the character handle, scenario name and flags (arguments 0-2) to
/// the engine implementation. Returns whatever the engine call returned.
export!(cdecl, rw_00bbabd0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
