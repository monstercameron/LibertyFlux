// original: 0x00b87050 POINT_CAM_AT_PED
/// Script native `POINT_CAM_AT_PED` (hash 0x495B0B6F).
///
/// Forwards two script arguments (a camera handle and a character
/// handle) to the engine. No return slot is written.
export!(cdecl, rw_00b87050(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
