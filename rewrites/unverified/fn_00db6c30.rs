// original: 0x00DB6C30 input_control_state_clear_events

/// Clear the pending control-event state in the receiver: the timer word at
/// +0x40, the value word at +0x04, the byte flags at +0x0C and +0x18, and
/// the event code word at +0x10. This is a zero-argument thiscall method.
lf_checker_rt::export!(thiscall, rw_00db6c30(this: u32) -> u32 {
    unsafe {
        const TIMER: u32 = 0x40;
        const VALUE: u32 = 0x04;
        const VALUE_VALID: u32 = 0x0c;
        const EVENT_CODE: u32 = 0x10;
        const EVENT_PENDING: u32 = 0x18;

        ((this + TIMER) as *mut u32).write_unaligned(0);
        ((this + VALUE) as *mut u32).write_unaligned(0);
        ((this + VALUE_VALID) as *mut u8).write(0);
        ((this + EVENT_CODE) as *mut u32).write_unaligned(0);
        ((this + EVENT_PENDING) as *mut u8).write(0);
    }
    0
});
