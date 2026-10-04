// original: 0x00bb2530 IS_PLAYER_PLAYING
/// Script native handler `IS_PLAYER_PLAYING` (hash 0x08274BA4).
///
/// Forwards script argument 0 to the engine worker and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2530(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let answer: u32 = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
