// original: 0x009cbe20 IS_MOBILE_PHONE_CALL_ONGOING
/// Script native `IS_MOBILE_PHONE_CALL_ONGOING` (hash 0x698F6172).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores the low byte of its answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_009cbe20(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
