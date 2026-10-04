// original: 0x00b86fd0 POINT_CAM_AT_CAM
/// Script native `POINT_CAM_AT_CAM` (hash 0x44717CF9).
///
/// Forwards two script arguments (two camera handles) to the engine. No return slot is written.
export!(cdecl, rw_00b86fd0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
