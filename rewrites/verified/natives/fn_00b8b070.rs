// original: 0x00b8b070 GET_MODEL_NAME_FOR_DEBUG
/// Script native `GET_MODEL_NAME_FOR_DEBUG` (hash 0x4342350C).
///
/// Forwards one script argument (a model hash) to the engine and stores its
/// full 32-bit answer into the return slot. Unlike the boolean natives,
/// this handler keeps the whole answer (`mov`, not `movzx`).
export!(cdecl, rw_00b8b070(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
