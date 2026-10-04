// original: 0x009cbe00 IS_MISSION_COMPLETE_PLAYING
/// Script native `IS_MISSION_COMPLETE_PLAYING` (hash 0x6C3B5917).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_009cbe00(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
