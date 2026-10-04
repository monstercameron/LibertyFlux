// original: 0x00b86f50 IS_SCREEN_FADING_IN
/// Script native `IS_SCREEN_FADING_IN` (hash 0x5D1425DF).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86f50(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
