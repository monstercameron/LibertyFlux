// original: 0x00bd8230 NETWORK_DISPLAY_HOST_GAMER_CARD
/// Script native `NETWORK_DISPLAY_HOST_GAMER_CARD` (hash 0x414E4E7F).
///
/// Forwards one script argument (a network player index) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd8230(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
