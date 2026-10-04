// original: 0x00b86b10 GET_CAM_FAR_DOF
/// Script native `GET_CAM_FAR_DOF` (hash 0x1CB27FE1).
///
/// Forwards two script arguments (a camera handle and an out-pointer) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b86b10(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
