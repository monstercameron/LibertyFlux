// original: 0x00bb1cc0 DELETE_PLAYER
/// Script native `DELETE_PLAYER` (hash 0x627A3586).
///
/// Forwards one script argument (a player index) to the engine. No return slot is written.
export!(cdecl, rw_00bb1cc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
