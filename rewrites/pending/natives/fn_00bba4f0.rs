// original: 0x00bba4f0 TASK_SEEK_COVER_FROM_PED
/// Script native `TASK_SEEK_COVER_FROM_PED` (hash 0x2D9C3D5E).
///
/// Two ped handles + flag.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00bba4f0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2))
    }
});
