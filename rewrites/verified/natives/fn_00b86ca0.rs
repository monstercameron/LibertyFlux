// original: 0x00b86ca0 GET_RADAR_VIEWPORT_ID
/// Script native `GET_RADAR_VIEWPORT_ID` (hash 0x4A7C19FE).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00b86ca0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
