// original: 0x00bb6490 PLAYSTATS_CHEAT
/// Script native `PLAYSTATS_CHEAT` (hash 0x0F9B3A1C).
///
/// Forwards one script argument (a cheat id) to the stat-packet builder and sender. No return slot is written.
export!(cdecl, rw_00bb6490(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
