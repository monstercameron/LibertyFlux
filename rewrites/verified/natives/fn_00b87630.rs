// original: 0x00b87630 SET_CAM_ROT
/// Script native `SET_CAM_ROT`.
///
/// Forwards four script arguments to the engine: one integer followed by
/// three float bit-patterns. Floats are copied as raw bits. No return
/// slot is written.
export!(cdecl, rw_00b87630(ctx: *const u8) -> u32 {
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
