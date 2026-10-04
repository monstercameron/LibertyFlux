// original: 0x00b8d2d0 PRINT_WITH_6_NUMBERS
/// Script native `PRINT_WITH_6_NUMBERS` (hash 0x03A01F39).
///
/// Forwards nine script arguments (a text label, a duration and seven number
/// slots) to the engine. No return slot is written.
///
/// Note: the checker's call log records the first eight stack arguments, so
/// the ninth forwarded word is covered by the rewrite's construction, not by
/// the outgoing-call comparison (same limit as the earlier wide-call
/// verifications).
export!(cdecl, rw_00b8d2d0(ctx: *const u8) -> u32 {
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
            *args.add(5),
            *args.add(6),
            *args.add(7),
            *args.add(8),
        )
    }
});
