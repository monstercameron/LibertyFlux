// original: 0x00BDF030 CTaskComplexUseAttractor::CTaskComplexUseAttractor

/// Initialize an attractor-use task with one identifier at byte offset `0x14`, after the base task initializer, and return the object pointer.
lf_checker_rt::export!(thiscall, rw_00bdf030(this: u32, attractor_id: u32) -> u32 {
unsafe {
    let task = this as *mut u8;
    let _base_result = lf_checker_rt::callee_thiscall!(1, u32, this);
    task.cast::<u32>().write(lf_checker_rt::relocated(0x00eb8f94));
    task.add(0x14).cast::<u32>().write(attractor_id);
    this
}
});
