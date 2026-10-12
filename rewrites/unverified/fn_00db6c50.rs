// original: 0x00DB6C50 input_control_state_set_first_enabled

/// When the low byte of the stack argument enables this mode, clear the pending value and copy the current timer into the receiver. The global timer value is loaded at call time. ECX is the receiver,
/// the stack argument is caller supplied, and the callee removes one word.
lf_checker_rt::export!(thiscall, rw_00db6c50(this: u32, enabled_word: u32) -> u32 {
    unsafe {
        const ENABLED_FLAG: u32 = 0x1c;
        const TIMER_GLOBAL: u32 = 0x01173594;
        ((this + ENABLED_FLAG) as *mut u8).write(enabled_word as u8);
        if enabled_word as u8 != 0 {
        const VALUE: u32 = 0x04;
        const TIMER: u32 = 0x3C;
        ((this + VALUE) as *mut u32).write_unaligned(0);
        let timer = lf_checker_rt::global::<u32>(TIMER_GLOBAL).read_unaligned();
        ((this + TIMER) as *mut u32).write_unaligned(timer);
        }
    }
    0
});
