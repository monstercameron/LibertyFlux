// original: 0x00b9af80 SWITCH_ROADS_ON
/// Script native `SWITCH_ROADS_ON` (hash 0x56553F38).
///
/// Forwards six float bit-patterns (two positions) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9af80(ctx: *const u8) -> u32 {
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
            *args.add(5),
        );
        answer
    }
});
