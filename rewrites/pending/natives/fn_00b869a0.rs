// original: 0x00b869a0 DETACH_CAM_FROM_VIEWPORT
/// Script native `DETACH_CAM_FROM_VIEWPORT` (hash 0x1DEA65DE).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00b869a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
