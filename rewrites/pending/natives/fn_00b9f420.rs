// original: 0x00b9f420 GET_OFFSET_FROM_CHAR_IN_WORLD_COORDS
/// Script native `GET_OFFSET_FROM_CHAR_IN_WORLD_COORDS` (hash 0x737F24F9).
///
/// Reads a character-relative offset in world coordinates: forwards a
/// character handle, three float bit-patterns (the offset) and three
/// more words to the engine. No return slot is written.
export!(cdecl, rw_00b9f420(ctx: *const u8) -> u32 {
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
