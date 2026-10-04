// original: 0x00bb9060 TASK_AIM_GUN_AT_COORD
/// Script native `TASK_AIM_GUN_AT_COORD` (hash 0x0AA202B0).
///
/// Forwards five script arguments to the engine: a character handle, three
/// float bit-patterns (target coordinates), and an integer duration. Floats
/// are copied as raw bits, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00bb9060(ctx: *const u8) -> u32 {
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
