// original: 0x00ba25a0 SET_PED_NON_REMOVAL_AREA
/// Script native `SET_PED_NON_REMOVAL_AREA` (hash 0x52D34ED3).
///
/// Forwards six script arguments (float bit-patterns, two 3D corners) to
/// the engine by value. Floats are copied as raw bits, so the forward is
/// bit-exact. No return slot is written. (The original pre-arranges the
/// arguments in its stack frame, with below-ESP temporaries, instead of
/// pushing them one by one; the observable call is an ordinary forward.)
export!(cdecl, rw_00ba25a0(ctx: *const u8) -> u32 {
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
            *args.add(5),
        )
    }
});
