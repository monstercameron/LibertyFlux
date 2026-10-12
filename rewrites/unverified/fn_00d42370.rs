// original: 0x00D42370 task_node_a8_reset

/// Clear the timer word at +0x80 and the active byte at +0xa4 in an object
/// passed in ECX. No other bytes change; EAX returns the same pointer.
lf_checker_rt::export!(thiscall, rw_D42370(this: u32) -> u32 {
    const TIMER: u32 = 0x80;
    const ACTIVE: u32 = 0xa4;
    unsafe {
        core::ptr::write_unaligned((this + TIMER) as *mut u32, 0);
        core::ptr::write((this + ACTIVE) as *mut u8, 0);
    }
    this
});
