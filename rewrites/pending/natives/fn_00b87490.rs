// original: 0x00b87490 SET_CAM_MOTION_BLUR
/// Script native `SET_CAM_MOTION_BLUR` (hash 0x693D7B21).
///
/// Forwards two script arguments (a camera handle and a blur amount) to the engine.
///
/// The amount is a float, forwarded as raw bits. No return slot is written.
export!(cdecl, rw_00b87490(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
