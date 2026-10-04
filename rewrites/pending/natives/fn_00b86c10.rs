// original: 0x00b86c10 GET_CINEMATIC_CAM
/// Script native `GET_CINEMATIC_CAM` (hash 0x00C87FB8).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00b86c10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
