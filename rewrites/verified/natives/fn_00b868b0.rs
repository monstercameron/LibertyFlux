// original: 0x00b868b0 CAM_SET_INTERP_STATE_SRC
/// Script native `CAM_SET_INTERP_STATE_SRC` (hash 0x32C67124).
///
/// Forwards two script arguments (a camera handle and a state value) to
/// the engine. No return slot is written.
export!(cdecl, rw_00b868b0(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const u32;
        callee_cdecl!(1, u32, *args, *args.add(1))
    }
});
