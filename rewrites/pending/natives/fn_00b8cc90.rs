// original: 0x00b8cc90 IS_FONT_LOADED
/// Script native `IS_FONT_LOADED` (hash 0x69B53ADA).
///
/// Forwards arg0 to the engine.
/// Writes the zero-extended low byte of the engine answer into the return slot.
export!(cdecl, rw_00b8cc90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ans = callee_cdecl!(1, u32, *args.add(0));
        let slot = (*(ctx as *const u32)) as *mut u32;
        *slot = ans & 0xFF;
        slot as u32
    }
});
