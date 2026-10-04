// original: 0x00b9e5d0 COPY_ANIMATIONS
/// Script native `COPY_ANIMATIONS` (hash 0x308D1778).
///
/// Forwards three script arguments (two character handles and a blend
/// factor) to the engine. The third argument travels through an SSE
/// register in the original, but only its bit pattern is copied, so the
/// rewrite forwards it as an integer word: bit-exact by construction.
export!(cdecl, rw_00b9e5d0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
