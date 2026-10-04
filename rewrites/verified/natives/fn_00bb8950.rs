// original: 0x00bb8950 ATTACH_PED_TO_SHIMMY_EDGE
/// Script native `ATTACH_PED_TO_SHIMMY_EDGE` (hash 0x0860560B).
///
/// Forwards five script arguments (a character handle and four float bit-patterns) to the engine. No return slot is written.
export!(cdecl, rw_00bb8950(ctx: *const u8) -> u32 {
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
        )
    }
});
