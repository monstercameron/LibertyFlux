// original: 0x009cb9c0 CANCEL_CURRENTLY_PLAYING_AMBIENT_SPEECH
/// Script native `CANCEL_CURRENTLY_PLAYING_AMBIENT_SPEECH` (hash 0x495D445F).
///
/// Forwards arg0 to the engine.
/// No return slot is written.
export!(cdecl, rw_009cb9c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0));
        ans
    }
});
