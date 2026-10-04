// original: 0x00bd9570 SET_RICH_PRESENCE_TEMPLATEMP2
/// Script native `SET_RICH_PRESENCE_TEMPLATEMP2` (hash 0x5AFA67D7).
///
/// Forwards one script argument (a presence template index) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd9570(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
