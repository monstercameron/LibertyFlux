// original: 0x00b868e0 CLONE_CAM
/// Script native `CLONE_CAM` (hash 0x483E5BE8).
///
/// Forwards two script arguments (a camera handle and flags) to the engine.
/// No return slot is written.
export!(cdecl, rw_00b868e0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
