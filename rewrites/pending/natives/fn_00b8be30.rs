// original: 0x00b8be30 CHANGE_PICKUP_BLIP_SPRITE
/// Script native `CHANGE_PICKUP_BLIP_SPRITE` (hash 0x05766DDE).
///
/// Forwards one script argument to the engine. No return slot is written.
export!(cdecl, rw_00b8be30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
