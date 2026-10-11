// original: 0x00B88510 NativeImpl_GET_CAM_ROT_2

/// Forward a camera identifier to the camera service and copy the returned
/// three floating point words into the caller's output pointers.
lf_checker_rt::export!(cdecl, rw_00b88510(camera_id: u32, out_x: u32, out_y: u32, out_z: u32) -> () {
    unsafe {
        let mut angles = [0u32; 3];
        let _ = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(0x0128e400), camera_id, angles.as_mut_ptr() as u32);
        core::ptr::write_unaligned(out_x as *mut u32, angles[0]);
        core::ptr::write_unaligned(out_y as *mut u32, angles[1]);
        core::ptr::write_unaligned(out_z as *mut u32, angles[2]);
    }
});
