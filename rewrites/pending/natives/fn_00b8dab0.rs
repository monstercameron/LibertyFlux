// original: 0x00b8dab0 SET_TEXT_VIEWPORT_ID
/// Script native `SET_TEXT_VIEWPORT_ID` (hash 0x3F9B2DD6).
///
/// Forwards one script argument (a viewport id) to the engine. No return slot is written.
export!(cdecl, rw_00b8dab0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
