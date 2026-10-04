// original: 0x00b8bd80 CHANGE_BLIP_SCALE
/// Script native `CHANGE_BLIP_SCALE` (hash 0x44D349D9).
///
/// Forwards two script arguments (a blip handle and a scale factor) to the engine.
///
/// The scale is a float, forwarded as raw bits. No return slot is written.
export!(cdecl, rw_00b8bd80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
