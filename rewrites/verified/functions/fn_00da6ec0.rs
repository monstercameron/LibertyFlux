// original: 0x00DA6EC0 CTaskComplexSmartFleeEntity::vf1

/// Resolve the task manager, then create a smart-flee task from the stored
/// identifier, two selector bytes, and the task's float and word fields.
/// After construction, copy this task's word at `+0x44` into the result's
/// `+0x44` field and return that result. The checked contract exercises the
/// non-null manager route; the original's null-manager branch writes through
/// address `0x44` and is outside this proof.
lf_checker_rt::export!(thiscall, rw_00DA6EC0(this: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL_VA: u32 = 0x0167E2A0;
        const MANAGER_LOOKUP: u32 = 1;
        const TASK_FACTORY: u32 = 2;
        const TASK_IDENTIFIER: u32 = 0x14;
        const SELECTOR_A: u32 = 0x38;
        const FLOAT_34: u32 = 0x34;
        const WORD_30: u32 = 0x30;
        const WORD_3C: u32 = 0x3C;
        const FLOAT_40: u32 = 0x40;
        const SELECTOR_B: u32 = 0x39;
        const COPIED_FIELD: u32 = 0x44;

        let manager_context = lf_checker_rt::global::<u32>(MANAGER_GLOBAL_VA).read_unaligned();
        let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, manager_context);
        if manager == 0 {
            return 0;
        }

        let task_identifier = (this.wrapping_add(TASK_IDENTIFIER) as *const u32).read_unaligned();
        let selector_a = (this.wrapping_add(SELECTOR_A) as *const u8).read() as u32;
        let float_34_bits = (this.wrapping_add(FLOAT_34) as *const u32).read_unaligned();
        let word_30 = (this.wrapping_add(WORD_30) as *const u32).read_unaligned();
        let word_3c = (this.wrapping_add(WORD_3C) as *const u32).read_unaligned();
        let float_40_bits = (this.wrapping_add(FLOAT_40) as *const u32).read_unaligned();
        let selector_b = (this.wrapping_add(SELECTOR_B) as *const u8).read() as u32;
        let new_task = lf_checker_rt::callee_thiscall!(
            TASK_FACTORY, u32, manager, task_identifier, selector_a, float_34_bits,
            word_30, word_3c, float_40_bits, selector_b
        );
        let copied_value = (this.wrapping_add(COPIED_FIELD) as *const u32).read_unaligned();
        (new_task.wrapping_add(COPIED_FIELD) as *mut u32).write_unaligned(copied_value);
        new_task
    }
});
