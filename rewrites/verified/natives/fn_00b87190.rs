// original: 0x00b87190 RESET_CAM_INTERP_CUSTOM_SPEED_GRAPH
/// Script native `RESET_CAM_INTERP_CUSTOM_SPEED_GRAPH` (hash 0x779F3EC6).
///
/// The handler body is a single jump to a shared implementation taking the same call context. The rewrite expresses that as a forwarding call with identical arguments and result.
export!(cdecl, rw_00b87190(ctx: *const u8) -> u32 {
    unsafe {
        callee_cdecl!(1, u32, ctx as u32)
    }
});
