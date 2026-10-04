// original: 0x00bd8150 NETWORK_AM_I_MUTED_BY_PLAYER
/// Script native `NETWORK_AM_I_MUTED_BY_PLAYER` (hash 0x448F486A).
///
/// Forwards one script argument (a network player index) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd8150(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
