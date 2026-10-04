// original: 0x00b86860 CAM_SET_INTERPOLATION_DETAILS
/// Script native `CAM_SET_INTERPOLATION_DETAILS` (hash 0x5AAC39C1).
///
/// Forwards one script argument (an interpolation handle) to the engine. No return slot is written.
export!(cdecl, rw_00b86860(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
