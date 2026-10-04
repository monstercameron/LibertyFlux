// original: 0x00ba21f0 SET_NM_ANIM_POSE
/// Script native `SET_NM_ANIM_POSE`.
///
/// Forwards four script arguments to the engine: three integers followed
/// by one float bit-pattern. No return slot is written.
export!(cdecl, rw_00ba21f0(ctx: *const u8) -> u32 {
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

