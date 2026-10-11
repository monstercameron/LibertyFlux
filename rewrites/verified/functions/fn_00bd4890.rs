// original: 0x00bd4890 NativeImpl_DRAW_CHECKPOINT

/// Forward seven single-precision argument words to the checkpoint renderer.
/// The first three words are snapshotted as a local vector; the renderer also
/// receives the remaining four words, a fixed mode value, and a zero flag.
/// The direct cdecl callee's EAX is returned unchanged.
#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn rw_00bd4890(
    position_x: u32,
    position_y: u32,
    position_z: u32,
    radius: u32,
    target_x: u32,
    target_y: u32,
    target_z: u32,
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
            0xb2,
            0,
        )
    }
}
