// original: 0x00B886C0 NativeImpl_GET_VIEWPORT_POS_AND_SIZE_2

/// Query the camera service and copy its four returned vector words into the
/// caller's output pointers.
lf_checker_rt::export!(cdecl, rw_00b886c0(camera_id: u32, out_0: u32, out_1: u32, out_2: u32, out_3: u32) -> () {
    unsafe {
        let mut values = [0u32; 4];
        let _ = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(0x0128e400), camera_id,
            values.as_mut_ptr() as u32, values.as_mut_ptr().add(2) as u32);
        core::ptr::write_unaligned(out_0 as *mut u32, values[0]);
        core::ptr::write_unaligned(out_1 as *mut u32, values[1]);
        core::ptr::write_unaligned(out_2 as *mut u32, values[2]);
        core::ptr::write_unaligned(out_3 as *mut u32, values[3]);
    }
});
