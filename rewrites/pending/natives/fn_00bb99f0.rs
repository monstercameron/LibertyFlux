// original: 0x00bb99f0 TASK_GOTO_CHAR_AIMING
/// Script native `TASK_GOTO_CHAR_AIMING` (hash 0x65EB71CC).
///
/// Forwards four script arguments to the engine: two character handles and
/// two float bit-patterns (a range and a height). No return slot is written;
/// the engine answer is the exit value.
export!(cdecl, rw_00bb99f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
