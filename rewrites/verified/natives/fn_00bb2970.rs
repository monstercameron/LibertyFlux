// original: 0x00bb2970 SET_MAX_WANTED_LEVEL
/// Script native `SET_MAX_WANTED_LEVEL` (hash 0x5D622498).
///
/// Forwards one script argument (the wanted level) to the engine. No
/// return slot is written.
export!(cdecl, rw_00bb2970(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
