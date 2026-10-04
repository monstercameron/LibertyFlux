// original: 0x00b87ca0 SET_SNIPER_ZOOM_FACTOR
/// Script native `SET_SNIPER_ZOOM_FACTOR` (hash 0x42690F6B).
///
/// Forwards one float script argument (the zoom factor) to the engine,
/// forwarded as a raw `u32` bit-pattern so it matches bit-exactly by
/// construction. No return slot is written.
export!(cdecl, rw_00b87ca0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
