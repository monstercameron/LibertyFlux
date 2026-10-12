// original: 0x00BDF210 CTaskSimpleMoveStandStill::CTaskSimpleMoveStandStill

/// Runs the base float-task constructor with a zero flag, then initializes the
/// stand-still task's relocated vtables, integer and byte fields, copied float
/// bits, and three zero-valued state fields. The thiscall receiver is returned
/// in EAX; four 32-bit argument slots are popped.
lf_checker_rt::export!(thiscall, rw_bdf210(
    this: u32,
    task_id: u32,
    option_a: u32,
    option_b: u32,
    float_bits: u32,
) -> u32 {
    unsafe {
        const FLOAT_TASK_CTOR: u32 = 1;
        const TASK_VTABLE: u32 = 0x00EB8C3C;
        const TASK_SUB_VTABLE_ADDRESS: u32 = 0x00EB8C90;
        const TASK_SUB_VTABLE_FIELD: u32 = 0x14;
        const TASK_ID: u32 = 0x30;
        const FLOAT_VALUE: u32 = 0x34;
        const OPTION_A: u32 = 0x38;
        const OPTION_B: u32 = 0x39;
        const STATE_WORD_A: u32 = 0x3c;
        const STATE_WORD_B: u32 = 0x40;
        const STATE_HALFWORD: u32 = 0x44;

        let _ = lf_checker_rt::callee_thiscall!(FLOAT_TASK_CTOR, u32, this, 0u32);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(TASK_VTABLE));
        (this.wrapping_add(TASK_SUB_VTABLE_FIELD) as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(TASK_SUB_VTABLE_ADDRESS));
        (this.wrapping_add(TASK_ID) as *mut u32).write_unaligned(task_id);
        (this.wrapping_add(FLOAT_VALUE) as *mut u32).write_unaligned(float_bits);
        (this.wrapping_add(OPTION_A) as *mut u8).write(option_a as u8);
        (this.wrapping_add(OPTION_B) as *mut u8).write(option_b as u8);
        (this.wrapping_add(STATE_WORD_A) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_WORD_B) as *mut u32).write_unaligned(0);
        (this.wrapping_add(STATE_HALFWORD) as *mut u16).write_unaligned(0);
        this
    }
});
