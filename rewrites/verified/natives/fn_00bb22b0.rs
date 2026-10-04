// original: 0x00bb22b0 HAS_PLAYER_RANK_BEEN_UPGRADED
/// Script native `HAS_PLAYER_RANK_BEEN_UPGRADED` (hash 0x6A842382).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00bb22b0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
