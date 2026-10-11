// original: 0x00B888F0 NativeImpl_CAM_IS_SPHERE_VISIBLE

/// Forward the identifier and four point words to the camera service; the
/// callee determines visibility. The pointer targets a frame-local snapshot.
lf_checker_rt::export!(cdecl, rw_00b888f0(handle: u32, x: u32, y: u32, z: u32, radius: u32) -> () {
    let point = [x, y, z, radius];
    let _ = lf_checker_rt::callee_thiscall!(1, u32,
        lf_checker_rt::relocated(0x0128e400), handle, point.as_ptr() as u32);
});
