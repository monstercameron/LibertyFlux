// original: 0x00b9e940 DELETE_CHAR
/// Script native `DELETE_CHAR` (hash 0x0E3B49BF).
///
/// Forwards one script argument (a character handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9e940(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
