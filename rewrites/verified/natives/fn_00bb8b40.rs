// original: 0x00bb8b40 FLUSH_COVER_BLOCKING_AREAS
/// Script native `FLUSH_COVER_BLOCKING_AREAS` (hash 0x5A535133).
///
/// Tail-jumps to a 2-instruction stub that zeroes the cover-blocking-area count global.
///
/// Script arguments: 1 word(s).
///
/// The handler body is a single tail jump to a shared engine routine;
/// this rewrite forwards the call context and returns the result,
/// which is what the tail-jump interception compares.
export!(cdecl, rw_00bb8b40(ctx: *const u8) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, ctx as u32)
    }
});
