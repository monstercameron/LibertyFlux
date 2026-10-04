// original: 0x00bb74e0 MARK_MODEL_AS_NO_LONGER_NEEDED
/// Script native `MARK_MODEL_AS_NO_LONGER_NEEDED` (hash 0x00FA0E33).
///
/// Forwards one script argument (a model hash) to the engine. No return slot is written.
export!(cdecl, rw_00bb74e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
