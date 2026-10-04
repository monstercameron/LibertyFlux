// original: 0x00bb1ef0 GET_PLAYER_COLOUR
/// Script native `GET_PLAYER_COLOUR` (hash 0x25270A4B).
///
/// Forwards one script argument (a player index) to the engine and stores
/// the full 32-bit answer (a colour value) into the return slot. The answer
/// is the exit value.
export!(cdecl, rw_00bb1ef0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
