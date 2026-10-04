// original: 0x00b9e330 BLEND_OUT_CHAR_MOVE_ANIMS
/// Script native `BLEND_OUT_CHAR_MOVE_ANIMS` (hash 0x65A34B7A).
///
/// Forwards one script argument (a character handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b9e330(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
