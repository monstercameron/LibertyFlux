// original: 0x00bd0d60 CLEAR_CHAR_LAST_WEAPON_DAMAGE
/// Script native `CLEAR_CHAR_LAST_WEAPON_DAMAGE` (hash 0x718508B4).
///
/// Forwards one script argument (a character handle) to the engine. No
/// return slot is written.
export!(cdecl, rw_00bd0d60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
