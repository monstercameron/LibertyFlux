// original: 0x00b9e850 CREATE_RANDOM_CHAR_AS_PASSENGER
/// Script native `CREATE_RANDOM_CHAR_AS_PASSENGER` (hash 0x46D01849).
///
/// Forwards three script arguments (vehicle handle and seat selectors) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b9e850(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
