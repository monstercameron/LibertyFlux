// original: 0x00bb8bc0 GET_PED_CLIMB_STATE
/// Script native `GET_PED_CLIMB_STATE` (hash 0x391822A7).
///
/// Forwards one script argument (a character handle) to the engine and
/// stores its full 32-bit answer (the climb state) into the return slot.
export!(cdecl, rw_00bb8bc0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});

