// original: 0x00bdbdd0 RESTORE_SCRIPT_VALUES_FOR_NETWORK_GAME
/// Script native `RESTORE_SCRIPT_VALUES_FOR_NETWORK_GAME` (hash 0x37CD55AA).
///
/// Adds 4 to the first script argument (a pointer to a value block) and
/// passes the adjusted pointer to the engine, then stores the low byte of
/// the engine answer (zero-extended) into the return slot.
export!(cdecl, rw_00bdbdd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, (*args).wrapping_add(4));
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
