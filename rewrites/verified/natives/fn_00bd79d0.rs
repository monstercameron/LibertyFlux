// original: 0x00bd79d0 GET_GFWL_IS_RETURNING_TO_SINGLE_PLAYER
/// Script native `GET_GFWL_IS_RETURNING_TO_SINGLE_PLAYER`
/// (hash 0x2FDF565D).
///
/// Takes no script arguments: calls the engine predicate with no arguments
/// and stores the low byte of its answer (zero-extended) into the return
/// slot.
export!(cdecl, rw_00bd79d0(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
