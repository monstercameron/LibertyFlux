// original: 0x00bba550 TASK_SEEK_COVER_TO_COORDS
/// Script native `TASK_SEEK_COVER_TO_COORDS` (hash 0x142F31EF).
///
/// Tasks a character with seeking cover at coordinates. Forwards eight
/// script arguments to the engine: the character handle, six float
/// bit-patterns (coordinates, copied as raw bits) and an integer. No return
/// slot is written.
export!(cdecl, rw_00bba550(ctx: *const u8) -> u32 {
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
            *args.add(7),
        )
    }
});
