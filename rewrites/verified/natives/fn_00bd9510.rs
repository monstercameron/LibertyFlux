// original: 0x00bd9510 SET_RICH_PRESENCE
/// Script native `SET_RICH_PRESENCE` (hash 0x73AB2028).
///
/// Forwards five script arguments (presence template parameters) to the
/// engine. No return slot is written.
export!(cdecl, rw_00bd9510(ctx: *const u8) -> u32 {
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
