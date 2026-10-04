// original: 0x00bb9330 TASK_CHAR_SLIDE_TO_COORD
/// Script native `TASK_CHAR_SLIDE_TO_COORD` (hash 0x04962F82).
///
/// Forwards six script arguments (a character handle, three float
/// bit-patterns for the target coordinates, a heading and a flag word) to
/// the engine. Floats are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
/// (The original stages five of the words through local stack slots below
/// its frame; the checker does not observe that scratch, only the outgoing
/// call's arguments, which match exactly.)
export!(cdecl, rw_00bb9330(ctx: *const u8) -> u32 {
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
            *args.add(5),
        )
    }
});
