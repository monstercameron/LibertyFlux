// original: 0x00b9e8b0 CREATE_RANDOM_MALE_CHAR
/// Script native `CREATE_RANDOM_MALE_CHAR` (hash 0x2FC728BB).
///
/// Forwards four script arguments to the engine: three float bit-patterns
/// (spawn coordinates) and one integer. No return slot is written.
export!(cdecl, rw_00b9e8b0(ctx: *const u8) -> u32 {
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
