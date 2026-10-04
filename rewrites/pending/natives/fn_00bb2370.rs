// original: 0x00bb2370 IS_IN_PLAYER_SETTINGS_MENU
/// Script native `IS_IN_PLAYER_SETTINGS_MENU` (hash 0x18CA2D3A).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb2370(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
