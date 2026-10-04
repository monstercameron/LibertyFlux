// original: 0x00bb9f80 TASK_LOOK_AT_CHAR
/// Script native `TASK_LOOK_AT_CHAR` (hash 0x2DD35B3F).
///
/// Forwards four script arguments (character handles and task parameters)
/// to the engine. No return slot is written.
export!(cdecl, rw_00bb9f80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
