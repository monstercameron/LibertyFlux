// original: 0x00bd7cb0 GET_TEAM_COLOUR
/// Script native `GET_TEAM_COLOUR` (hash 0x4FC96A24).
///
/// Forwards one script argument (a team index) to the engine and stores its full 32-bit answer (a colour value) into the return slot.
export!(cdecl, rw_00bd7cb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
