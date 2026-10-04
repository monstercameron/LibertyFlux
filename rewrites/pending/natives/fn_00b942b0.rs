// original: 0x00b942b0 FORCE_WIND
/// Script native `FORCE_WIND` (hash 0x310E75C9).
///
/// Forwards arg0 (float bits) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b942b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0));
        ans
    }
});
