// original: 0x00b9e740 CREATE_DUMMY_CHAR
/// Script native `CREATE_DUMMY_CHAR` (hash 0x44FF276B).
///
/// Spawns a dummy character: forwards ten script words (a model word,
/// five float bit-patterns for coordinates and heading, and four more
/// words) to the engine. Floats are copied as raw bits. No return slot
/// is written.
export!(cdecl, rw_00b9e740(ctx: *const u8) -> u32 {
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
            *args.add(8),
            *args.add(9),
        )
    }
});
