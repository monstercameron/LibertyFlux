// original: 0x0086f530 CLEAR_TEXT_LABEL
/// Script native `CLEAR_TEXT_LABEL` (hash 0x412E68D0).
///
/// Writes zero through the pointer held in script argument 0, clearing the
/// caller's text label slot. This handler makes no engine call; it is a
/// single store. Returns the pointer it wrote through (the exit value of
/// the original's address register).
export!(cdecl, rw_0086f530(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let target = *args as *mut u32;
        *target = 0;
        *args
    }
});
