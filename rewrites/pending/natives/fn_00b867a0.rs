// original: 0x00b867a0 CAM_SEQUENCE_GET_PROGRESS
/// Script native `CAM_SEQUENCE_GET_PROGRESS` (hash 0x7AAD273F).
///
/// Forwards two script arguments (a camera-sequence handle and an out
/// value) to the engine. No return slot is written by the handler itself.
export!(cdecl, rw_00b867a0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
