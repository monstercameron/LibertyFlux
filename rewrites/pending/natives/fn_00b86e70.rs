// original: 0x00b86e70 IS_CAM_SHAKING
/// Script native `IS_CAM_SHAKING` (hash 0x089C57D7).
///
/// Takes no script arguments: calls the engine worker with no arguments and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86e70(ctx: *const u8) -> u32 {
    unsafe {
        let slot = *(ctx as *const u32) as *mut u32;
        let answer = callee_cdecl!(1, u32,);
        *slot = answer & 0xFF;
        slot as u32
    }
});
