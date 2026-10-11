// original: 0x00C954C0 SetTaskValueAndResetState

/// Set bit 3 in the receiver's state word at +0x5C, store the first stack word at +0x28, clear the dwords at +0x54 and +0x20, and return the input value in EAX. The thiscall epilogue removes one stack word.
lf_checker_rt::export!(thiscall, rw_00c954c0(this: u32, value: u32) -> u32 {
    const STATE_FLAGS: u32 = 0x5C;
    const VALUE: u32 = 0x28;
    const STATE_VALUE: u32 = 0x54;
    const SECONDARY_STATE: u32 = 0x20;
    const ACTIVE_FLAG: u32 = 8;
    let state = unsafe { (this.wrapping_add(STATE_FLAGS) as *const u32).read_unaligned() };
    unsafe {
        (this.wrapping_add(STATE_FLAGS) as *mut u32).write_unaligned(state | ACTIVE_FLAG);
        (this.wrapping_add(VALUE) as *mut u32).write_unaligned(value);
        (this.wrapping_add(STATE_VALUE) as *mut u32).write_unaligned(0);
        (this.wrapping_add(SECONDARY_STATE) as *mut u32).write_unaligned(0);
    }
    value
});
