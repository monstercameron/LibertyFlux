// original: 0x00b87670 SET_CAM_SHAKE
/// Script native `SET_CAM_SHAKE` (hash 0x686B6395).
///
/// Forwards three script arguments (a camera handle and two parameters)
/// to the engine. No return slot is written.
export!(cdecl, rw_00b87670(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
