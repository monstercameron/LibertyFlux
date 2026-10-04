// original: 0x00B86890 CAM_SET_INTERP_GRAPH_ROT
/// Sets the camera interpolation graph rotation from two script arguments.
///
/// Forwards both argument words to the engine camera function and returns
/// its answer unchanged.
lf_rn26_rt::export!(cdecl, rw_00B86890(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        lf_rn26_rt::callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
