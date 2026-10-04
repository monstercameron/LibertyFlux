// original: 0x00b86930 CREATE_CAM
/// Script native `CREATE_CAM` (hash 0x694A0DC1).
///
/// Forwards two script arguments to the engine. No return slot is written.
export!(cdecl, rw_00b86930(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
