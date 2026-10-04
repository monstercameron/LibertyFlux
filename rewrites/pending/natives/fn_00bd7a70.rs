// original: 0x00bd7a70 GET_LCPD_CRIMINAL_SCORE
/// Script native `GET_LCPD_CRIMINAL_SCORE` (hash 0x1207025C).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores all 32 bits of its answer into the return slot.
export!(cdecl, rw_00bd7a70(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer;
        answer
    }
});
