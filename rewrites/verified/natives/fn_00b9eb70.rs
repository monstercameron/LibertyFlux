// original: 0x00b9eb70 FORCE_SPAWN_SCENARIO_PEDS_IN_AREA
/// Script native `FORCE_SPAWN_SCENARIO_PEDS_IN_AREA` (hash 0x186D42A4).
///
/// Forwards five script arguments (three float coordinates, a float radius and an integer). The call passes eight words: the five arguments followed by a repeat of the first three coordinate words (the handler's float scratch sits inside the caller's cleanup range, so all eight words are observed call arguments) to the engine. No return slot is written.
export!(cdecl, rw_00b9eb70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
            *args,
            *args.add(1),
            *args.add(2),
        )
    }
});
