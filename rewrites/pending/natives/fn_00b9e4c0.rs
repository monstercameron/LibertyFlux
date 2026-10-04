// original: 0x00b9e4c0 CLEAR_ALL_CHAR_RELATIONSHIPS
/// Script native `CLEAR_ALL_CHAR_RELATIONSHIPS` (hash 0x57297D58).
///
/// Forwards two script arguments (two character handles) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9e4c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
