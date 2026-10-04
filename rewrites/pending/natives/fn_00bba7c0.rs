// original: 0x00bba7c0 TASK_SIT_DOWN_INSTANTLY
/// Script native `TASK_SIT_DOWN_INSTANTLY` (hash 0x6CC1560F).
///
/// Forwards four script arguments (a character handle and three integers)
/// to the engine. No return slot is written.
export!(cdecl, rw_00bba7c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
