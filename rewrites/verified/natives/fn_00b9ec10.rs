// original: 0x00b9ec10 GET_ANIM_GROUP_FROM_CHAR
/// Script native `GET_ANIM_GROUP_FROM_CHAR` (hash 0x55EB748F).
///
/// Forwards one script argument (a character handle) to the engine and
/// /// stores its full 32-bit answer (an animation-group id) into the
/// /// return slot. Unlike the boolean natives, this handler keeps the
/// /// whole answer (`mov`, not `movzx`).
export!(cdecl, rw_00b9ec10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
