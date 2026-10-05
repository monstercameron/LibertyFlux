// original: 0x00b8d810 SET_PLAYER_ICON_COLOUR
/// Script native `SET_PLAYER_ICON_COLOUR` (hash 0x689D5EEE).
///
/// Forwards one script argument (a colour dword) to the engine. No return
/// slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00b8d810(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
