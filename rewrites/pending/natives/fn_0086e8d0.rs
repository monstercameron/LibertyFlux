// original: 0x0086e8d0 SETTIMERB
/// Script native `SETTIMERB` (hash 0x3B4C2E2E).
///
/// Takes one script argument (the new timer value) and stores it at
/// offset 0x20 of the game-owned timer structure reached through a
/// global pointer. It makes no engine call at all.
export!(cdecl, rw_0086e8d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        // The script timer lives in a game-owned structure reached through a
        // global pointer; the new value goes at offset 0x20. No engine call.
        let timer_struct = *global::<u32>(0x01BB54DC);
        *((timer_struct + 0x20) as *mut u32) = *args;
        timer_struct
    }
});
