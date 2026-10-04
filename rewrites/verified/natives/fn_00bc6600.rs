// original: 0x00BC6600 HAS_RESPRAY_HAPPENED
// Rewrite of the HAS_RESPRAY_HAPPENED native handler.

/// Script native `HAS_RESPRAY_HAPPENED()`.
///
/// Takes no script arguments: calls the engine respray-status routine and
/// stores the zero-extended low byte of its answer in the context's return
/// slot.
export!(cdecl, rw_bc6600(ctx: *const u8) -> u32 {
    unsafe {
        let answer: u32 = callee_cdecl!(1, u32,);
        let ret_slot = *(ctx as *const *mut u32);
        *ret_slot = answer & 0xFF;
        0
    }
});
