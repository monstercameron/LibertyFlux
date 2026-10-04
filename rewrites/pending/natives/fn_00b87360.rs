// original: 0x00b87360 SET_CAM_FAR_DOF
/// Script native `SET_CAM_FAR_DOF` (hash 0x52F543A3).
///
/// Forwards 2 script arguments to the engine in order.
/// Float arguments are forwarded as raw bit patterns (bit-exact).
/// No return slot is written.
export!(cdecl, rw_00b87360(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
