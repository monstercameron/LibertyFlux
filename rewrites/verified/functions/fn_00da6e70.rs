// original: 0x00DA6E70 CTaskComplexFleeShooting::vf1

/// Resolve the task manager and request a shooting-flee task when it exists.
/// The factory receives the identifier at `+0x14`, zero-extended selector
/// byte at `+0x38`, float words at `+0x34` and `+0x40`, and stored words at
/// `+0x30`, `+0x60`, `+0x64`, and `+0x3C`, in that order. The float words are
/// forwarded without arithmetic so their bit patterns remain exact.
lf_checker_rt::export!(thiscall, rw_00DA6E70(this: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL_VA: u32 = 0x0167E2A0;
        const MANAGER_LOOKUP: u32 = 1;
        const TASK_FACTORY: u32 = 2;
        const TASK_IDENTIFIER: u32 = 0x14;
        const SELECTOR: u32 = 0x38;
        const FLOAT_34: u32 = 0x34;
        const FLOAT_40: u32 = 0x40;
        const WORD_30: u32 = 0x30;
        const WORD_60: u32 = 0x60;
        const WORD_64: u32 = 0x64;
        const WORD_3C: u32 = 0x3C;

        let manager_context = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
        let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, manager_context);
        if manager == 0 {
            return 0;
        }

        let task_identifier = (this.wrapping_add(TASK_IDENTIFIER) as *const u32).read_unaligned();
        let selector = (this.wrapping_add(SELECTOR) as *const u8).read() as u32;
        let float_34_bits = (this.wrapping_add(FLOAT_34) as *const u32).read_unaligned();
        let word_30 = (this.wrapping_add(WORD_30) as *const u32).read_unaligned();
        let word_60 = (this.wrapping_add(WORD_60) as *const u32).read_unaligned();
        let word_64 = (this.wrapping_add(WORD_64) as *const u32).read_unaligned();
        let word_3c = (this.wrapping_add(WORD_3C) as *const u32).read_unaligned();
        let float_40_bits = (this.wrapping_add(FLOAT_40) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            TASK_FACTORY, u32, manager, task_identifier, selector, float_34_bits,
            word_30, word_60, word_64, word_3c, float_40_bits
        )
    }
});
