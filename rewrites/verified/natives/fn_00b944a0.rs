// original: 0x00b944a0 GET_EPISODE_NAME
/// Script native `GET_EPISODE_NAME` (hash 0x6004431B).
///
/// Forwards one script argument (an episode index) to the engine and stores
/// the full 32-bit answer (a string pointer) into the return slot.
export!(cdecl, rw_00b944a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32, *args);
        *slot = answer;
        answer
    }
});
