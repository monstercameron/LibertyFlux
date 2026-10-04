// original: 0x00bb1af0 ALTER_WANTED_LEVEL
/// Script native `ALTER_WANTED_LEVEL` (hash 0x60C80EC9).
///
/// Player index + wanted level.
/// No return slot is written.
///
/// The handler returns the engine's answer in EAX (the register
/// is untouched after the call); the rewrite does the same.
export!(cdecl, rw_00bb1af0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
