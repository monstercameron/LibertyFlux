// original: 0x00bd89a0 NETWORK_JOIN_GAME
/// Script native `NETWORK_JOIN_GAME` (hash 0x60806A0C).
///
/// Forwards one script argument to the engine and stores the low byte of
/// its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd89a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
