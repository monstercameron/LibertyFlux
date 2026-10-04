// original: 0x00bb75a0 REQUEST_INTERIOR_MODELS
/// Script native `REQUEST_INTERIOR_MODELS` (hash 0x302E113D).
///
/// Forwards two script arguments (an interior id and flags) to the engine.
/// No return slot is written.
export!(cdecl, rw_00bb75a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
