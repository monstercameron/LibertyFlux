// original: 0x00bb9000 SET_SEQUENCE_TO_REPEAT
/// Script native `SET_SEQUENCE_TO_REPEAT` (hash 0x22E91F1F).
///
/// Forwards 2 script arguments to the engine. No return slot is written.
export!(cdecl, rw_00bb9000(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
