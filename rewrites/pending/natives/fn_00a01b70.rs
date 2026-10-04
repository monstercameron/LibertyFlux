// original: 0x00a01b70 SET_OBJECT_ALPHA
/// Script native `SET_OBJECT_ALPHA` (hash 0x7F0040DE).
///
/// Forwards two script arguments (an object handle and an alpha value)
/// to the engine. No return slot is written.
export!(cdecl, rw_00a01b70(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
