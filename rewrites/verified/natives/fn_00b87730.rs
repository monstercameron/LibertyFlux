// original: 0x00b87730 SET_CAMERA_STATE
/// Script native `SET_CAMERA_STATE` (hash 0x4ED45146).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b87730(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
