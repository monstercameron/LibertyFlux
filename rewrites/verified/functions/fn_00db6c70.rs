// original: 0x00DB6C70 input_control_state_set_second_enabled

/// When enabled, clear the event code and copy the current timer; event code 17 also clears the related auxiliary word. The global timer value is loaded at call time. ECX is the receiver,
/// the stack argument is caller supplied, and the callee removes one word.
lf_checker_rt::export!(thiscall, rw_00db6c70(this: u32, enabled_word: u32) -> u32 {
    unsafe {
        const ENABLED_FLAG: u32 = 0x1d;
        const TIMER_GLOBAL: u32 = 0x01173594;
        ((this + ENABLED_FLAG) as *mut u8).write(enabled_word as u8);
        if enabled_word as u8 != 0 {
        const EVENT_CODE: u32 = 0x10;
        const AUXILIARY: u32 = 0x48;
        const TIMER: u32 = 0x3C;
        let code = ((this + EVENT_CODE) as *const u32).read_unaligned();
        if code == 17 {
            ((this + AUXILIARY) as *mut u32).write_unaligned(0);
        }
        ((this + EVENT_CODE) as *mut u32).write_unaligned(0);
        let timer = lf_checker_rt::global::<u32>(TIMER_GLOBAL).read_unaligned();
        ((this + TIMER) as *mut u32).write_unaligned(timer);
        }
    }
    0
});
