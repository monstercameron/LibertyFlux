// original: 0x00bd41c0 REMOVE_SPHERE
/// Script native `REMOVE_SPHERE` (hash 0x12A909C9).
///
/// Forwards one script argument (a sphere handle) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bd41c0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
