// original: 0x00b8be10 CHANGE_PICKUP_BLIP_SCALE
/// Script native `CHANGE_PICKUP_BLIP_SCALE` (hash 0x4F66544E).
///
/// Forwards one script argument (a float scale factor, copied as raw bits)
/// to the engine. No return slot is written.
export!(cdecl, rw_00b8be10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
