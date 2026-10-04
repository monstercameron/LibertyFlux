// original: 0x009cbdc0 IS_GAME_IN_CONTROL_OF_MUSIC
/// Script native `IS_GAME_IN_CONTROL_OF_MUSIC` (hash 0x4FF71989).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_009cbdc0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
