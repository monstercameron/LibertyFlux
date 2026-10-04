// original: 0x00b9e4b0 CLEAR_ALL_CHAR_PROPS
/// Script native `CLEAR_ALL_CHAR_PROPS` (hash 0x232A52FA).
///
/// Forwards one script argument (a character handle) to the engine.
export!(cdecl, rw_00b9e4b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
