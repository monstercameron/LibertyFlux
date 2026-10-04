// original: 0x00b94080 CLEAR_AREA_OF_CHARS
/// Script native `CLEAR_AREA_OF_CHARS` (hash 0x0C2747B9).
///
/// Forwards arg0 (float bits), arg1 (float bits), arg2 (float bits), arg3 (float bits) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b94080(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0), *args.add(1), *args.add(2), *args.add(3));
        ans
    }
});
