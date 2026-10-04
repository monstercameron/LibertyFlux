// original: 0x00bd40a0 GET_TXD
/// Script native `GET_TXD` (hash 0x15D668D0).
///
/// Forwards one script argument (a texture-dictionary handle) to the engine
/// and stores the full 32-bit engine answer into the return slot. The
/// handler leaves the engine answer in EAX (it addresses the slot through
/// ECX), so the exit value is the answer, not the slot pointer.
export!(cdecl, rw_00bd40a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
