// original: 0x00b86890 CAM_SET_INTERP_GRAPH_ROT
use lf_k2_rt::{callee_cdecl, export};
/// Sets the camera interpolation graph rotation from two script arguments.
///
/// Forwards both argument words to the engine camera function and returns
/// its answer unchanged.
export!(cdecl, rw_00B86890(ctx: *const u8) -> u32 {
    unsafe {
        let a = *(ctx.add(8) as *const *const u32);
        callee_cdecl!(1, u32, *a, *a.add(1))
    }
});
