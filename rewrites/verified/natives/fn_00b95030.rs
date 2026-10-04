// original: 0x00b95030 SET_TIME_SCALE
/// Script native `SET_TIME_SCALE` (hash 0x24D467CC).
///
/// Forwards one float bit-pattern (the time scale) to the engine. The
/// value is copied as raw bits, so the forward is bit-exact. No return
/// slot is written.
export!(cdecl, rw_00b95030(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
