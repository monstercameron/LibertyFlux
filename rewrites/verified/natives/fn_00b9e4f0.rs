// original: 0x00b9e4f0 CLEAR_CHAR_LAST_DAMAGE_ENTITY
/// Script native `CLEAR_CHAR_LAST_DAMAGE_ENTITY` (hash 0x0AB9317B).
///
/// Forwards one script argument (a character handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9e4f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
