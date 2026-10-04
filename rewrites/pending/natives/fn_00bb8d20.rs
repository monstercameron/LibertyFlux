// original: 0x00bb8d20 IS_CHAR_GETTING_UP
// Rewrite of the IS_CHAR_GETTING_UP native handler.

/// Script native `IS_CHAR_GETTING_UP(char)`.
///
/// Forwards the character handle to the engine getting-up check and stores
/// the zero-extended low byte of its answer in the context's return slot.
export!(cdecl, rw_bb8d20(ctx: *const u8) -> u32 {
    unsafe {
        let args = *((ctx.add(8)) as *const *const u32);
        let answer: u32 = callee_cdecl!(1, u32, *args);
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        0
    }
});
