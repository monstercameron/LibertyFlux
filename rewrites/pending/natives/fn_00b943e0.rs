// original: 0x00b943e0 GET_CURRENT_WEATHER_FULL
/// Script native `GET_CURRENT_WEATHER_FULL` (hash 0x3FFA65EE).
///
/// Forwards three script arguments to the engine. No return slot is
/// written.
export!(cdecl, rw_00b943e0(ctx: *const u8) -> u32 {
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
