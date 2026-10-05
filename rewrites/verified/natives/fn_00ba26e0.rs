// original: 0x00ba26e0 SET_PLAYER_SETTINGS_GENRE
/// Script native `SET_PLAYER_SETTINGS_GENRE` (hash 0x379B0A8F).
///
/// Forwards one script argument (a genre id) to the engine. No return slot
/// is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00ba26e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
