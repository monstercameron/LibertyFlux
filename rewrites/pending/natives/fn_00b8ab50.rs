// original: 0x00b8ab50 HAS_CUTSCENE_LOADED
/// Script native `HAS_CUTSCENE_LOADED` (hash 0x5DE43980).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores the low byte of its answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00b8ab50(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32, );
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
