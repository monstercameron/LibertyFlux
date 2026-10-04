// original: 0x00b8cd30 IS_PAUSE_MENU_ACTIVE
/// Script native `IS_PAUSE_MENU_ACTIVE` (hash 0x6C4568A7).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b8cd30(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
