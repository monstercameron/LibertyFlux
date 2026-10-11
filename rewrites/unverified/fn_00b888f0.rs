// original: 0x00B888F0 NativeImpl_CAM_IS_SPHERE_VISIBLE

/// Forward an identifier and three point words to the camera service; the
/// callee determines visibility. The trailing radius input is not passed.
lf_checker_rt::export!(cdecl, rw_00b888f0(handle: u32, x: u32, y: u32, z: u32, _radius: u32) -> () {
    let point = [x, y, z];
    let _ = lf_checker_rt::callee_thiscall!(1, u32,
        lf_checker_rt::relocated(0x0128e400), handle, point.as_ptr() as u32);
});
