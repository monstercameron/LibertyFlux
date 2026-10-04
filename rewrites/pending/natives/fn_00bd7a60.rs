// original: 0x00bd7a60 GET_LCPD_COP_SCORE
/// Script native `GET_LCPD_COP_SCORE` (hash 0x17C05D83).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the full 32-bit answer into the return slot.
export!(cdecl, rw_00bd7a60(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer;
        answer
    }
});
