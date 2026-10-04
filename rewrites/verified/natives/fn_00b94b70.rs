// original: 0x00b94b70 IS_THIS_A_MINIGAME_SCRIPT
/// Script native `IS_THIS_A_MINIGAME_SCRIPT` (hash 0x219A3AF6).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b94b70(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
