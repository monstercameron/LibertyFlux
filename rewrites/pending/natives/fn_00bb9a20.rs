// original: 0x00bb9a20 TASK_GOTO_CHAR_OFFSET
/// Script native `TASK_GOTO_CHAR_OFFSET` (hash 0x658028BA).
///
/// Forwards five script arguments to the engine: three integers (handles)
/// followed by two float bit-patterns (offsets, copied as raw bits). No
/// return slot is written.
export!(cdecl, rw_00bb9a20(ctx: *const u8) -> u32 {
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
