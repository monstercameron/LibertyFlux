// original: 0x00b8ce90 LOCK_PLAYER_SETTINGS_GENRE_CHANGE
/// Script native `LOCK_PLAYER_SETTINGS_GENRE_CHANGE` (hash 0x33F4498E).
///
/// Forwards one script argument to the engine. No return slot is written. (The original cleans its one pushed argument with `(an instruction of the original)`; the effect on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b8ce90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
