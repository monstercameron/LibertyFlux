// original: 0x00bb9d30 TASK_GUARD_SPHERE_DEFENSIVE_AREA
/// Script native `TASK_GUARD_SPHERE_DEFENSIVE_AREA` (hash 0x01795753).
///
/// Forwards eleven script arguments (character handle plus coordinates and parameters, mostly floats as raw bits) to the engine task routine. The original builds the argument block with an SSE shuffle that leaves six dead spill words above it; only the eleven live words reach the engine. Writes no return slot.
export!(cdecl, rw_00bb9d30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5), *args.add(6), *args.add(7), *args.add(8), *args.add(9), *args.add(10))
    }
});
