// original: 0x00bd40e0 LOAD_TXD
/// Script native `LOAD_TXD` (hash 0x52FC763A).
///
/// Forwards one script argument (the texture-dictionary name pointer) to
/// the engine and stores the full 32-bit answer into the return slot.
export!(cdecl, rw_00bd40e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
