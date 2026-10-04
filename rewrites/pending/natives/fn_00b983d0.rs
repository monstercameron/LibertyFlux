// original: 0x00b983d0 GET_POSITION_OF_ANALOGUE_STICKS
/// Script native `GET_POSITION_OF_ANALOGUE_STICKS` (hash 0x4F7F4FAE).
///
/// Forwards five script arguments (a pad index and four out-pointers) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b983d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
