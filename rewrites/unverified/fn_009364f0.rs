// original: 0x009364F0 reset_network_object_counter

/// Clear the dword at object offset `0xd7c`, then tail-call the base
/// transition helper with the same object. The helper's return value becomes
/// this function's return value. This is a thiscall routine with no stack
/// arguments.
lf_checker_rt::export!(thiscall, rw_009364f0(this: u32) -> u32 {
    unsafe {
        const COUNTER_FIELD: u32 = 0x0d7c;
        unsafe { (this.wrapping_add(COUNTER_FIELD) as *mut u32).write_unaligned(0) };
        lf_checker_rt::callee_thiscall!(1, u32, this)
    }
});
