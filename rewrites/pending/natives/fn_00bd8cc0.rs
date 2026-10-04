// original: 0x00bd8cc0 NETWORK_SET_MATCH_PROGRESS
/// Script native `NETWORK_SET_MATCH_PROGRESS` (hash 0x5C8D66EA).
///
/// Forwards one script word (match progress) to the engine. No return
/// slot is written.
export!(cdecl, rw_00bd8cc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
