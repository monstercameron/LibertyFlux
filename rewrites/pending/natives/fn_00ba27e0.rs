// original: 0x00ba27e0 SET_SCENARIO_PED_DENSITY_MULTIPLIER
/// Script native `SET_SCENARIO_PED_DENSITY_MULTIPLIER`.
///
/// Forwards two float bit-patterns to the engine. No return slot is written.
export!(cdecl, rw_00ba27e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
