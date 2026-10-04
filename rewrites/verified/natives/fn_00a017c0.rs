// original: 0x00a017c0 MARK_OBJECT_AS_NO_LONGER_NEEDED
/// Script native `MARK_OBJECT_AS_NO_LONGER_NEEDED` (hash 0x493B655B).
///
/// Forwards one script argument to the engine.
/// No return slot is written.
export!(cdecl, rw_00a017c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
