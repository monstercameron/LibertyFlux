// original: 0x00ba2ab0 WARP_CHAR_FROM_CAR_TO_CAR
/// Script native `WARP_CHAR_FROM_CAR_TO_CAR` (hash 0x3AE77439).
///
/// Forwards three script arguments (a character handle and two vehicle handles) to the engine.
/// No return slot is written.
export!(cdecl, rw_00ba2ab0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2),)
    }
});
