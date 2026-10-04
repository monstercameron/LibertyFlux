// original: 0x00b86bb0 GET_CAM_POS
/// Script native `GET_CAM_POS` (hash 0x60C22E93).
///
/// Forwards four script arguments (a camera handle and three out-pointers)
/// to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b86bb0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1), *args.add(2), *args.add(3))
    }
});
