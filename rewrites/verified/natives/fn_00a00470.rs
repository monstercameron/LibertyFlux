// original: 0x00a00470 ATTACH_OBJECT_TO_CAR
/// Script native `ATTACH_OBJECT_TO_CAR` (hash 0x7E81412A).
///
/// Forwards nine script arguments to the engine: three integers (handles
/// and flags) followed by six float bit-patterns (two coordinate triples).
/// The original shuffles the words through two stack temporaries, but the
/// callee receives them in script order, so the forward is a plain copy.
/// Floats are copied as raw bits, so the forward is bit-exact. No return
/// slot is written.
export!(cdecl, rw_00a00470(ctx: *const u8) -> u32 {
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
        )
    }
});
