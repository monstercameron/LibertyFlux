// original: 0x00bd7df0 IS_IN_MP_TUTORIAL
/// Script native `IS_IN_MP_TUTORIAL` (hash 0x13750991).
///
/// Calls the engine tutorial-state query with no arguments and stores the
/// low byte of the answer (a boolean) into the return slot.
export!(cdecl, rw_00bd7df0(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer & 0xFF;
        slot as u32
    }
});
