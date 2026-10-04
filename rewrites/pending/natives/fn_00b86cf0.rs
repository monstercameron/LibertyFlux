// original: 0x00b86cf0 GET_SCRIPT_DRAW_CAM
/// Script native `GET_SCRIPT_DRAW_CAM` (hash 0x30F71BC6).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00b86cf0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
