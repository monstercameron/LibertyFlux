// original: 0x00cfc120 CTaskComplexCombatAdditionalTask::CTaskComplexCombatAdditionalTask
/// Call the complex-task base constructor, then store the task type and
/// parameter-block pointer at offsets 0x14 and 0x18. Copy three vector words
/// to offsets 0x20 through 0x28; store the maximum word, factor, flags, and zeroed
/// fields through offset 0x44. If the parameter block pointer is non-null, call the
/// reference helper with it and the stored pointer field. Return this object.
lf_checker_rt::export!(thiscall, rw_00cfc120(this: *mut u8, task_type: u32, parameter_block: *const u32, vector: *const u32, factor_bits: u32, flags: u32) -> u32 {
    unsafe {
        const TASK_VTABLE: u32 = 0x00EE0484;
        lf_checker_rt::callee_thiscall!(1, u32, this as u32);
        *(this.add(0x14) as *mut u32) = task_type;
        *(this.add(0x18) as *mut u32) = parameter_block as u32;
        *(this as *mut u32) = lf_checker_rt::relocated(TASK_VTABLE);
        let vector_words = vector;
        for index in 0..3 {
            *(this.add(0x20 + index * 4) as *mut u32) = *vector_words.add(index);
        }
        *(this.add(0x30) as *mut u32) = u32::MAX;
        *(this.add(0x34) as *mut u32) = factor_bits;
        *(this.add(0x38) as *mut u32) = 0;
        *(this.add(0x3C) as *mut u32) = 0;
        *(this.add(0x40) as *mut u32) = flags;
        *(this.add(0x44) as *mut u32) = 0;
        if !parameter_block.is_null() {
            lf_checker_rt::callee_thiscall!(2, u32, parameter_block as u32, this.add(0x18) as u32);
        }
        this as u32
    }
});
