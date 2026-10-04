// original: 0x00b874b0 SET_CAM_NAME
/// Script native `SET_CAM_NAME` (hash 0x2AE87B02).
///
/// Forwards two script arguments (a camera handle and a name) to the
/// /// engine. No return slot is written.
export!(cdecl, rw_00b874b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
