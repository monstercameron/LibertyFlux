// original: 0x00bb2670 IS_SYSTEM_UI_SHOWING
/// Script native `IS_SYSTEM_UI_SHOWING` (hash 0x5F643EE6).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// Stores the low byte of the engine answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00bb2670(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
