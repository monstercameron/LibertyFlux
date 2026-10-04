// original: 0x00b8bc80 CHANGE_BLIP_ALPHA
/// Script native `CHANGE_BLIP_ALPHA` (hash 0x2FB14E41).
///
/// Forwards two script arguments (a blip handle and an alpha value) to
/// the engine worker. No return slot is written.
export!(cdecl, rw_00b8bc80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
