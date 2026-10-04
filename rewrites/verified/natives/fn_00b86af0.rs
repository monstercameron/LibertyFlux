// original: 0x00B86AF0 GET_CAM_FAR_CLIP
/// F03 GET_CAM_FAR_CLIP: forwards (args[0], args[1]), no return slot use.
export!(cdecl, rn10_get_cam_far_clip(ctx: *const u32) -> u32 {
    unsafe {
        let (_, args) = ctx_parts(ctx);
        callee_cdecl!(2, u32, *args, *args.add(1));
        0
    }
});
