// original: 0x00bb21c0 GIVE_REMOTE_CONTROLLED_MODEL_TO_PLAYER
/// Script native `GIVE_REMOTE_CONTROLLED_MODEL_TO_PLAYER` (hash 0x663979D9).
///
/// Forwards six script arguments (two integers around four float
/// bit-patterns) to the engine worker. No return slot is written.
export!(cdecl, rw_00bb21c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3), *args.add(4), *args.add(5))
    }
});
