// original: 0x00bd4100 MARK_STREAMED_TXD_AS_NO_LONGER_NEEDED
/// Script native `MARK_STREAMED_TXD_AS_NO_LONGER_NEEDED` (hash 0x70EA2B89).
///
/// Forwards one script argument (a texture-dictionary handle) to the
/// engine. No return slot is written.
/// (The original cleans its one pushed argument with `(an instruction of the original)`; the effect
/// on the stack pointer is identical to the plain cdecl return here.)
export!(cdecl, rw_00bd4100(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
