// original: 0x0087EA70 unnamed_0087ea70

/// Initialize the fixed fields of a buffer state and run its helper twice.
///
/// `this` addresses a 28-byte state. The constructor clears the first five
/// words, sets `+0x14` to the 256-byte limit and `+0x18` to `u32::MAX`, calls
/// the same no-argument thiscall helper twice, then sets `+0x0c` to four,
/// resets `+0x10` to zero and restores the limit. Each helper call is
/// intercepted by the checker. The constructor returns `this`.
///
/// Original: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_0087ea70(this: u32) -> u32 {
    const COUNTER_OFFSET: u32 = 0x0C;
    const POSITION_OFFSET: u32 = 0x10;
    const CAPACITY_OFFSET: u32 = 0x14;
    const LIMIT: u32 = 0x100;
    const INVALID_INDEX: u32 = u32::MAX;
    const INITIALIZE_HELPER: u32 = 1;

    unsafe {
        for offset in [0u32, 4, 8, COUNTER_OFFSET, POSITION_OFFSET] {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(CAPACITY_OFFSET) as *mut u32).write_unaligned(LIMIT);
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(INVALID_INDEX);
        let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_HELPER, u32, this);
        let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_HELPER, u32, this);
        (this.wrapping_add(COUNTER_OFFSET) as *mut u32).write_unaligned(4);
        (this.wrapping_add(POSITION_OFFSET) as *mut u32).write_unaligned(0);
        (this.wrapping_add(CAPACITY_OFFSET) as *mut u32).write_unaligned(LIMIT);
    }
    this
});
