// original: 0x00ba2790 SET_ROTATION_FOR_ATTACHED_PED
/// Script native `SET_ROTATION_FOR_ATTACHED_PED`.
///
/// Forwards four script arguments to the engine: one integer followed by
/// three float bit-patterns. Floats are copied as raw bits. No return
/// slot is written.
export!(cdecl, rw_00ba2790(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(
            1,
            u32,
            *args,
            *args.add(1),
            *args.add(2),
            *args.add(3),
        )
    }
});
