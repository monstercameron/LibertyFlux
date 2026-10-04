// original: 0x00b86f90 IS_SPECIFIC_CAM_INTERPOLATING
/// Script native `IS_SPECIFIC_CAM_INTERPOLATING` (hash 0x17C37E6D).
///
/// Forwards one script argument (a camera id) to the engine and stores the
/// low byte of its answer (zero-extended) into the return slot.
export!(cdecl, rw_00b86f90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        let answer = callee_cdecl!(1, u32, *args);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
