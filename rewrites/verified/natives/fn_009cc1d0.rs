// original: 0x009cc1d0 PLAY_STREAM_FROM_PED
/// Script native `PLAY_STREAM_FROM_PED` (hash 0x0C47057F).
///
/// Forwards one script argument (a character handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_009cc1d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
