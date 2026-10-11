// original: 0x00DA6E10 CTaskComplexFleeAnyMeans::vf1

/// Resolve the global task manager and request a flee task when it exists.
/// The factory receives the identifier at `+0x14`, byte `+0x19`, stored float
/// word `+0x28`, words `+0x2C` and `+0x1C`, word `+0x20`, float word `+0x24`,
/// word `+0x30`, and float word `+0x34`, in that order. All float values are
/// forwarded as their original words without arithmetic.
lf_checker_rt::export!(thiscall, rw_00DA6E10(this: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL_VA: u32 = 0x0167E2A0;
        const MANAGER_LOOKUP: u32 = 1;
        const TASK_FACTORY: u32 = 2;
        const TASK_IDENTIFIER: u32 = 0x14;
        const TASK_KIND: u32 = 0x19;
        const FLOAT_28: u32 = 0x28;
        const WORD_2C: u32 = 0x2C;
        const WORD_1C: u32 = 0x1C;
        const WORD_20: u32 = 0x20;
        const FLOAT_24: u32 = 0x24;
        const WORD_30: u32 = 0x30;
        const FLOAT_34: u32 = 0x34;

        let manager_context = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
        let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, manager_context);
        if manager == 0 {
            return 0;
        }

        let task_identifier = (this.wrapping_add(TASK_IDENTIFIER) as *const u32).read_unaligned();
        let task_kind = (this.wrapping_add(TASK_KIND) as *const u8).read() as u32;
        let float_28_bits = (this.wrapping_add(FLOAT_28) as *const u32).read_unaligned();
        let word_2c = (this.wrapping_add(WORD_2C) as *const u32).read_unaligned();
        let word_1c = (this.wrapping_add(WORD_1C) as *const u32).read_unaligned();
        let word_20 = (this.wrapping_add(WORD_20) as *const u32).read_unaligned();
        let float_24_bits = (this.wrapping_add(FLOAT_24) as *const u32).read_unaligned();
        let word_30 = (this.wrapping_add(WORD_30) as *const u32).read_unaligned();
        let float_34_bits = (this.wrapping_add(FLOAT_34) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            TASK_FACTORY, u32, manager, task_identifier, task_kind, float_28_bits,
            word_2c, word_1c, word_20, float_24_bits, word_30, float_34_bits
        )
    }
});
