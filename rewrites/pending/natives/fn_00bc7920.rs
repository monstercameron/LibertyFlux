// original: 0x00bc7920 SET_EXTRA_CAR_COLOURS
/// Script native `SET_EXTRA_CAR_COLOURS` (hash 0x6CB14354).
///
/// Forwards a car handle and two colour values to the engine. No return
/// slot is written.
export!(cdecl, rw_00bc7920(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
        )
    }
});
