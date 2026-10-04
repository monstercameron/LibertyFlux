// original: 0x00bc5f80 GET_MAXIMUM_NUMBER_OF_PASSENGERS
/// Script native `GET_MAXIMUM_NUMBER_OF_PASSENGERS` (hash 0x554014F1).
///
/// Forwards two script arguments to the engine passenger query. No return
/// slot is written.
export!(cdecl, rw_00bc5f80(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
