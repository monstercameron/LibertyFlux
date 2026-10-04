// original: 0x00b8d960 SET_TEXT_EDGE
/// Script native `SET_TEXT_EDGE` (hash 0x2D7A725D).
///
/// Forwards five script arguments (text edge style values) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8d960(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        );
        answer
    }
});
