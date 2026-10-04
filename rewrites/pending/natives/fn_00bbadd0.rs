// original: 0x00bbadd0 TASK_USE_NEAREST_SCENARIO_TO_POS_WARP
/// Script native `TASK_USE_NEAREST_SCENARIO_TO_POS_WARP` (hash 0x47787A40).
///
/// Forwards five script arguments to the engine in order: an integer
/// handle followed by four float bit-patterns (position and radius).
/// The original shuffles the floats through vector registers and scratch
/// stack slots; only the forwarded order matters here, and the floats are
/// copied as raw bits, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00bbadd0(ctx: *const u8) -> u32 {
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
        )
    }
});
