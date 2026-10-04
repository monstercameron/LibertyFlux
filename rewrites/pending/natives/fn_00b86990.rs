// original: 0x00b86990 DESTROY_VIEWPORT
/// Script native `DESTROY_VIEWPORT` (hash 0x651E50EC).
///
/// Forwards one script argument (a viewport handle) to the engine. No return slot is written.
export!(cdecl, rw_00b86990(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
