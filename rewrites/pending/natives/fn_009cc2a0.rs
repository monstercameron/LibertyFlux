// original: 0x009cc2a0 REPORT_CRIME
/// Script native `REPORT_CRIME` (hash 0x076B4C7C).
///
/// Forwards four script arguments to the engine: three float coordinates (copied as raw bits) and one integer. The original shuffles the floats through SSE temporaries on its own frame; the net effect is a plain four-word forward. No return slot is written.
export!(cdecl, rw_009cc2a0(ctx: *const u8) -> u32 {
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
