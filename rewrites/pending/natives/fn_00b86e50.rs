// original: 0x00b86e50 IS_CAM_SEQUENCE_COMPLETE
/// Script native `IS_CAM_SEQUENCE_COMPLETE` (hash 0x55727056).
///
/// Forwards one script word (a camera handle) to the engine and
/// stores the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86e50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
