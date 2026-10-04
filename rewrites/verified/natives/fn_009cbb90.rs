// original: 0x009cbb90 FORCE_RADIO_TRACK
/// Script native `FORCE_RADIO_TRACK` (hash 0x6A7E47C9).
///
/// Forwards four script arguments (station and track selectors) to the engine. No return slot is written.
export!(cdecl, rw_009cbb90(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
