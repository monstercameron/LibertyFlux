// original: 0x009cc2f0 REPORT_DISPATCH
/// Script native `REPORT_DISPATCH` (hash 0x388D6B44).
///
/// Forwards four script arguments (an integer and three float bit-patterns for a position) to the engine. The original builds the position through stack temporaries; only the four pushed words are observable. No return slot is written.
export!(cdecl, rw_009cc2f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
