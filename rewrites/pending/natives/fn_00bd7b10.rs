// original: 0x00bd7b10 GET_NETWORK_PLAYER_VIP
/// Script native `GET_NETWORK_PLAYER_VIP` (hash 0x0B6B0C10).
///
/// Forwards one script argument (a network player index) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd7b10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
