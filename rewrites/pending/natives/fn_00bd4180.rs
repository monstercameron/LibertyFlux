// original: 0x00bd4180 REMOVE_PTFX
/// Script native `REMOVE_PTFX` (hash 0x4AF643D5).
///
/// Forwards one script argument (a particle-effect handle) to the engine. No return slot is written.
export!(cdecl, rw_00bd4180(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args)
    }
});
