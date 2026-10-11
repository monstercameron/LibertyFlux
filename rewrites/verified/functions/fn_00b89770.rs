// original: 0x00B89770 NativeImpl_SET_FOLLOW_VEHICLE_CAM_OFFSET

/// Set the follow vehicle camera offset; pass the handle and an exact three-word coordinate snapshot to the service.
lf_checker_rt::export!(cdecl, rw_00b89770(handle: u32, x_bits: u32, y_bits: u32, z_bits: u32) -> () {
    const CAMERA_MANAGER: u32 = 0x0128e400;
    let point = [x_bits, y_bits, z_bits];
    let result = lf_checker_rt::callee_thiscall!(1, u32,
        lf_checker_rt::relocated(CAMERA_MANAGER), handle, point.as_ptr() as u32);
    let _ = lf_checker_rt::callee_thiscall!(2, u32,
        lf_checker_rt::relocated(CAMERA_MANAGER), result);
});

