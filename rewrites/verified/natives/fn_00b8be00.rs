// original: 0x00b8be00 CHANGE_PICKUP_BLIP_PRIORITY
/// Script native `CHANGE_PICKUP_BLIP_PRIORITY` (hash 0x31321D1A).
///
/// Forwards one script argument (a pickup handle) to the engine. No return slot is written.
export!(cdecl, rw_00b8be00(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
