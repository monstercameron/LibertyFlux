// original: 0x00b9ea00 DONT_REMOVE_CHAR
/// Script native `DONT_REMOVE_CHAR` (hash 0x3659084A).
///
/// Forwards one script argument (a character handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9ea00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
