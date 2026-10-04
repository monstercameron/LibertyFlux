// original: 0x00b87b50 SET_HINT_TIMES
/// Script native `SET_HINT_TIMES` (hash 0x4CC81FCB).
///
/// Forwards three script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b87b50(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
