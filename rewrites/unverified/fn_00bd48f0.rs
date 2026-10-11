// original: 0x00bd48f0 NativeImpl_DRAW_CHECKPOINT_WITH_ALPHA

/// Forward the checkpoint position, radius, target point, and alpha word to the renderer. The position is assembled into a three-word stack vector; the renderer's direct cdecl result is returned unchanged.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd48f0(
    position_x: u32,
    position_y: u32,
    position_z: u32,
    radius: u32,
    target_x: u32,
    target_y: u32,
    target_z: u32,
    alpha: u32,
) -> u32 {
    const DRAW_CALLEE: u32 = 1;
    let position = [position_x, position_y, position_z];
    unsafe {
        lf_checker_rt::callee_cdecl!(
            DRAW_CALLEE,
            u32,
            1,
            position.as_ptr() as usize as u32,
            radius,
            target_x,
            target_y,
            target_z,
            alpha,
            0,
        )
    }
}
