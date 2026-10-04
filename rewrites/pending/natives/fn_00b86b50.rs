// original: 0x00b86b50 GET_CAM_MOTION_BLUR
/// Script native `GET_CAM_MOTION_BLUR` (hash 0x64EF411D).
///
/// Forwards two script arguments (a camera handle and a blur value) to the engine. No return slot is written.
export!(cdecl, rw_00b86b50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
