// original: 0x00b87d60 SET_VIEWPORT
/// Script native `SET_VIEWPORT`.
///
/// Forwards five script arguments to the engine: one integer followed by
/// four float bit-patterns. Floats are copied as raw bits. No return
/// slot is written.
export!(cdecl, rw_00b87d60(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
            *args.add(4),
        )
    }
});
