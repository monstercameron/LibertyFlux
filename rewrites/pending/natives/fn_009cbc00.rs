// original: 0x009cbc00 GET_PLAYER_HAS_TRACKS
/// Script native `GET_PLAYER_HAS_TRACKS` (hash 0x396844BE).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
/// The handler reads only the return-slot pointer from the context.
export!(cdecl, rw_009cbc00(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
