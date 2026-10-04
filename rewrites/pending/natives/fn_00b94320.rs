// original: 0x00b94320 GENERATE_RANDOM_INT_IN_RANGE
/// Script native `GENERATE_RANDOM_INT_IN_RANGE` (hash 0x168B1717).
///
/// Forwards three script arguments to the engine. No return slot is
/// written by the handler itself.
export!(cdecl, rw_00b94320(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
