// original: 0x00ba2b30 WARP_CHAR_INTO_CAR_AS_PASSENGER
/// Script native `WARP_CHAR_INTO_CAR_AS_PASSENGER`.
///
/// Forwards three script arguments to the engine. No return slot is written.
export!(cdecl, rw_00ba2b30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});

