// original: 0x00C954A0 ClearTaskStateBitAndSetValue

/// Clear bit 3 in the receiver's state word at +0x5C, store the first stack word at +0x24, store the IEEE-754 bits for 1.0 at +0x54, and return the first stack word in EAX. The thiscall epilogue removes one stack word.
lf_checker_rt::export!(thiscall, rw_00c954a0(this: u32, value: u32) -> u32 {
    const STATE_FLAGS: u32 = 0x5C;
    const VALUE: u32 = 0x24;
    const STATE_VALUE: u32 = 0x54;
    const CLEAR_FLAG: u32 = 8;
    let state = unsafe { (this.wrapping_add(STATE_FLAGS) as *const u32).read_unaligned() };
    unsafe {
        (this.wrapping_add(STATE_FLAGS) as *mut u32).write_unaligned(state & !CLEAR_FLAG);
        (this.wrapping_add(VALUE) as *mut u32).write_unaligned(value);
        (this.wrapping_add(STATE_VALUE) as *mut u32).write_unaligned(1.0f32.to_bits());
    }
    value
});
