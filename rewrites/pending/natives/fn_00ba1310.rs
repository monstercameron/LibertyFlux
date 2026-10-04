// original: 0x00ba1310 SET_CHAR_DEFAULT_COMPONENT_VARIATION
/// Script native `SET_CHAR_DEFAULT_COMPONENT_VARIATION` (hash 0x4FB30DB6).
///
/// Forwards arg0 to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba1310(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0));
        ans
    }
});
