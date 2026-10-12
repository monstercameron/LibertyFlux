// original: 0x00c253c0 cam_interp_reset
/// Resets the interpolation bookkeeping after invoking the teardown helper. It clears the two source pointers and their counters, restores the invalid index sentinel, and returns the helper's exact result.
lf_checker_rt::export!(thiscall, rw_00c253c0(this: u32) -> u32 {
    unsafe {
        const SOURCE_A: usize = 0x14c;
        const SOURCE_B: usize = 0x150;
        const CURRENT_INDEX: usize = 0x144;
        const PREVIOUS_INDEX: usize = 0x148;
        const INVALID_INDEX: u32 = u32::MAX;
        let helper_result = lf_checker_rt::callee_thiscall!(0, u32, this);
        let object = this as usize as *mut u8;
        core::ptr::write_unaligned(object.add(SOURCE_A) as *mut u32, 0);
        core::ptr::write_unaligned(object.add(SOURCE_B) as *mut u32, 0);
        core::ptr::write_unaligned(object.add(CURRENT_INDEX) as *mut u32, 0);
        core::ptr::write_unaligned(object.add(PREVIOUS_INDEX) as *mut u32, INVALID_INDEX);
        helper_result
    }
});
