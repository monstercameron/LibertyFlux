// original: 0x00B88F10 NativeImpl_POINT_FIXED_CAM

/// Point the fixed camera at its target. The coordinates are copied bit-for-bit into a frame-local point
/// before the service call; its return word is forwarded to the follow-up.
lf_checker_rt::export!(cdecl, rw_00b88f10(x_bits: u32, y_bits: u32, z_bits: u32, extra: u32) -> () {
    const CAMERA_MANAGER: u32 = 0x0128e400;
    let point = [x_bits, y_bits, z_bits];
    let result = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(CAMERA_MANAGER), point.as_ptr() as u32, extra);
    let _ = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(CAMERA_MANAGER), result);
});
