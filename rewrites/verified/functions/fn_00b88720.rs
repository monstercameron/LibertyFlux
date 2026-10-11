// original: 0x00B88720 NativeImpl_HINT_CAM_2

/// Forward a camera hint and a three-float point to the camera service.
lf_checker_rt::export!(cdecl, rw_00b88720(x_bits: u32, y_bits: u32, z_bits: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> () {
    unsafe {
        let point = [x_bits, y_bits, z_bits];
        let _ = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(0x0128e400), point.as_ptr() as u32, a3, a4, a5, a6);
    }
});
