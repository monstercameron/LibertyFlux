// original: 0x00b86db0 IS_CAM_ACTIVE
/// Script native `IS_CAM_ACTIVE` (hash 0x348D7AF5).
///
/// Forwards one script argument (a camera handle) to the engine and stores
/// the low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86db0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
