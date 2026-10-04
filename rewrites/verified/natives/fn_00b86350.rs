// original: 0x00b86350 IS_WORLD_POINT_WITHIN_BRAIN_ACTIVATION_RANGE
/// Script native `IS_WORLD_POINT_WITHIN_BRAIN_ACTIVATION_RANGE` (hash 0x5E7B0F23).
///
/// Takes no script arguments: calls the engine worker with no arguments
/// and stores the low byte of its answer (zero-extended) into the
/// return slot.
export!(cdecl, rw_00b86350(ctx: *const u8) -> u32 {
    unsafe {
        let answer = callee_cdecl!(1, u32,);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
