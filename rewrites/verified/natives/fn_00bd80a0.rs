// original: 0x00bd80a0 LOCAL_PLAYER_IS_READY_TO_START_PLAYING
/// Script native `LOCAL_PLAYER_IS_READY_TO_START_PLAYING` (hash 0x5C03585C).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bd80a0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
