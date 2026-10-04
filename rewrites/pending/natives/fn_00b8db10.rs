// original: 0x00b8db10 SET_WIDESCREEN_FORMAT
/// Script native `SET_WIDESCREEN_FORMAT` (hash 0x7BDE2CAF).
///
/// Forwards one script argument (a format selector) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b8db10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
