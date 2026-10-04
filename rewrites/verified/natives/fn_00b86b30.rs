// original: 0x00b86b30 GET_CAM_FOV
/// Script native `GET_CAM_FOV` (hash 0x7BF4652D).
///
/// Forwards two script arguments (a camera handle and an out-pointer) to
/// the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b86b30(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
