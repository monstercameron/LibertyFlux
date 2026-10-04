// original: 0x00bb7520 REMOVE_IPL_DISCREETLY
/// Script native `REMOVE_IPL_DISCREETLY` (hash 0x658F21AF).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00bb7520(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
