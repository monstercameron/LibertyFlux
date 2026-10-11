// original: 0x00BDF120 CTaskComplexWaitAtAttractor::CTaskComplexWaitAtAttractor

/// Initialize a wait-at-attractor task with the attractor and mode words at offsets `0x14` and `0x18`, then return the object pointer.
lf_checker_rt::export!(thiscall, rw_00bdf120(this: u32, attractor: u32, wait_mode: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    task.add(0x14).cast::<u32>().write(attractor);
    task.add(0x18).cast::<u32>().write(wait_mode);
    task.cast::<u32>().write(lf_checker_rt::relocated(0x00eb8f3c));
    this
}
});
