// original: 0x00bb93c0 TASK_CHAR_SLIDE_TO_COORD_HDG_RATE
/// Script native `TASK_CHAR_SLIDE_TO_COORD_HDG_RATE` (hash 0x33D756A0).
///
/// Forwards seven script arguments to the engine: a character handle and
/// six float bit-patterns (target coordinates, heading and rate). Floats
/// are copied as raw bits, so the forward is bit-exact. No return slot is
/// written.
export!(cdecl, rw_00bb93c0(ctx: *const u8) -> u32 {
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
            *args.add(6),
        )
    }
});
