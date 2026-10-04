// original: 0x00b8bd60 CHANGE_BLIP_ROTATION
/// Script native `CHANGE_BLIP_ROTATION` (hash 0x3AF307B1).
///
/// Forwards two script arguments (a blip handle and a rotation value) to the
/// engine. No return slot is written.
export!(cdecl, rw_00b8bd60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
