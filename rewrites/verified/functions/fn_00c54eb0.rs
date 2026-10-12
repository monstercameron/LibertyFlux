// original: 0x00C54EB0 initialize_world_pool_header (proposed)
/// Initialize a world-pool header by calling the base initializer with
/// constant argument seven, setting its byte and scalar fields, and then
/// invoking the record-reset helper. Both direct callees are intercepted by
/// the proof contract. The method takes `this` in ECX, has no stack arguments,
/// and returns the original header pointer in EAX.
lf_checker_rt::export!(thiscall, rw_c54eb0(this: u32) -> u32 {
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this, 7u32);
        (this.wrapping_add(0x0c) as *mut u8).write(1);
        (this.wrapping_add(0x10) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x18) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0x1c) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(0);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this);
        this
    }
});
