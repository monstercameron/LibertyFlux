// original: 0x00ba26f0 SET_RELATIONSHIP
/// Script native `SET_RELATIONSHIP` (hash 0x03D916E4).
///
/// Forwards three script arguments (two groups and a relationship level) to the engine. No return slot is written.
export!(cdecl, rw_00ba26f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
