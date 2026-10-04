// original: 0x00bd40c0 HAS_STREAMED_TXD_LOADED
/// Script native `HAS_STREAMED_TXD_LOADED` (hash 0x5F9C43D4).
///
/// Forwards one script argument (a texture-dictionary reference) to the
/// engine and stores the low byte of its answer (zero-extended) into
/// the return slot.
export!(cdecl, rw_00bd40c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
