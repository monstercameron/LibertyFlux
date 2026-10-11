// original: 0x00B88E40 NativeImpl_POINT_FIXED_CAM_AT_POS

/// Set the fixed camera target position. The coordinates are copied bit-for-bit into a frame-local point
/// before the service call; its return word is forwarded to the follow-up.
lf_checker_rt::export!(cdecl, rw_00b88e40(x_bits: u32, y_bits: u32, z_bits: u32, extra: u32) -> () {
    const CAMERA_MANAGER: u32 = 0x0128e400;
    let point = [x_bits, y_bits, z_bits];
    let result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAMERA_MANAGER), point.as_ptr() as u32, extra);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(CAMERA_MANAGER), result);
});
