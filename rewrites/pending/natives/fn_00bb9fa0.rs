// original: 0x00bb9fa0 TASK_LOOK_AT_COORD
/// Script native `TASK_LOOK_AT_COORD` (hash 0x26E27605).
///
/// Forwards six script arguments (a character handle, a coordinate triple and two integers) to the engine.
///
/// The coordinates are floats, forwarded as raw bits. No return slot is written.
export!(cdecl, rw_00bb9fa0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5))
    }
});
