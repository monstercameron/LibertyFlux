// original: 0x00b94930 IS_EPISODIC_DISC_BUILD
/// Script native `IS_EPISODIC_DISC_BUILD` (hash 0x511A2EC9).
///
/// Takes no script arguments: calls the engine worker with no arguments and stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b94930(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
