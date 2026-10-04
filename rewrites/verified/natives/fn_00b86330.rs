// original: 0x00b86330 IS_OBJECT_WITHIN_BRAIN_ACTIVATION_RANGE
/// Script native `IS_OBJECT_WITHIN_BRAIN_ACTIVATION_RANGE` (hash 0x472C710B).
///
/// Forwards one script argument (an object handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86330(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
