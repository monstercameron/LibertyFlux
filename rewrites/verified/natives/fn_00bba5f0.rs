// original: 0x00bba5f0 TASK_SEEK_COVER_TO_OBJECT
/// Script native `TASK_SEEK_COVER_TO_OBJECT` (hash 0x4DB55DF5).
///
/// Forwards 6 script arguments (two handles, three coordinate floats and an integer) to the engine.
/// Float arguments are copied as raw bits, so the forward is bit-exact.
/// No return slot is written.
export!(cdecl, rw_00bba5f0(ctx: *const u8) -> u32 {
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
