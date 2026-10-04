// original: 0x00b98750 SHAKE_PAD_IN_CUTSCENE
/// Script native `SHAKE_PAD_IN_CUTSCENE` (hash 0x2D040DA9).
///
/// Forwards three script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b98750(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
        )
    }
});
