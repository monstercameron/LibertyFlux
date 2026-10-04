// original: 0x00bdb5a0 NETWORK_RESTORE_GAME_CONFIG
/// Script native `NETWORK_RESTORE_GAME_CONFIG` (hash 0x1E1B5C26).
///
/// Adds 4 to the pointer in script argument 0 and passes the adjusted pointer to the engine, then stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bdb5a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let ptr_plus4 = (*args).wrapping_add(4);
        let answer = callee_cdecl!(1, u32, ptr_plus4);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
