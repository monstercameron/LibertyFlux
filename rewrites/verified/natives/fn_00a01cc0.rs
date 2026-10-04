// original: 0x00a01cc0 SET_OBJECT_DRAW_LAST
/// Script native `SET_OBJECT_DRAW_LAST` (hash 0x19DD44F2).
///
/// Forwards an object handle and a flag (coerced with `arg != 0`) to the engine.
///
/// The flag rides in the low byte of a word whose upper bytes repeat
/// the context pointer (the original coerces it in its own stack slot);
/// the rewrite rebuilds that exact word from `ctx`.
export!(cdecl, rw_00a01cc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let flag = u32::from(*args.add(1) != 0);
        let quirked = (ctx as u32 & 0xFFFF_FF00) | flag;
        callee_cdecl!(1, u32, *args, quirked)
    }
});
