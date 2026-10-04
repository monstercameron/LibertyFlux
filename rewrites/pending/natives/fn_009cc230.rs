// original: 0x009cc230 PREVIEW_RINGTONE
/// Script native `PREVIEW_RINGTONE` (hash 0x79660015).
///
/// Forwards one ringtone index to the engine's audio worker. No return
/// slot is written.
export!(cdecl, rw_009cc230(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
