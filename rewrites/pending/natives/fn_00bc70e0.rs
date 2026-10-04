// original: 0x00bc70e0 PLAYER_IS_INTERACTING_WITH_GARAGE
/// Native handler `PLAYER_IS_INTERACTING_WITH_GARAGE`: calls its engine function and writes the
/// low byte of the result (zero-extended) to the return slot.
export!(cdecl, rw_00bc70e0(ctx: *const u32) -> u32 {
    unsafe {
        let slot = *ctx as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer & 0xFF;
        0
    }
});
