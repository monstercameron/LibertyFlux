// original: 0x00e43020 aligned_frame_draw_c
/// Rebuilds two rotated layers from per-trial extent and basis callbacks.
///
/// Each quarter-turn submits four corners with fixed texture coordinates.
/// When both computed bounds extend beyond the origin, a final clipped
/// rectangle is submitted. The function returns no value.
export!(thiscall, rw_00e43020(
    object: u32,
    offset_x: f32,
    offset_y: f32,
    limit_x: f32,
    limit_y: f32,
    payload: u32
) -> () {
    unsafe {
        run_aligned_frame_draw_c(object, offset_x, offset_y, limit_x, limit_y, payload, false)
    }
});
