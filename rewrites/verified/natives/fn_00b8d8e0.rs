// original: 0x00b8d8e0 SET_TEXT_CENTRE_WRAPX
/// Script native `SET_TEXT_CENTRE_WRAPX` (hash 0x2F9E362B).
///
/// Forwards one float argument (a wrap position) to the engine. It travels
/// through an SSE register in the original, but only its bit pattern is
/// copied, so the rewrite forwards it as an integer word: bit-exact by
/// construction.
export!(cdecl, rw_00b8d8e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
