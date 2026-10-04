// original: 0x00bb8850 ADD_COVER_POINT
/// Script native `ADD_COVER_POINT` (hash 0x18D5264D).
///
/// Forwards eight script arguments to the engine: four integers and four
/// float bit-patterns (coordinates) interleaved as stored in the argument
/// array. Floats are copied as raw bits, so the forward is bit-exact. No
/// return slot is written.
export!(cdecl, rw_00bb8850(ctx: *const u8) -> u32 {
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
