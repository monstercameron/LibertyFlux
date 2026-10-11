// original: 0x00dc8400 task_ctor_encoded_mode_and_weight

/// Construct a task with a copied three-word position, a target pointer and
/// an encoded mode. A helper's x87 `f32` result is passed as raw stack bits to
/// the base constructor. The mode updates selected bits at `+0x9c`, and a
/// relocated global float is copied to `+0x90` before the task initializer
/// runs. This thiscall routine cleans four stack words and returns `this`.
lf_checker_rt::export!(thiscall, rw_00dc8400(this_ptr: u32, random_input: u32, source_ptr: u32, target: u32, mode: u32) -> u32 {
    unsafe {
        const PRIMARY_VTABLE_VA: u32 = 0x00ef4e1c;
        const SECONDARY_VTABLE_VA: u32 = 0x00ef4e74;
        const INITIAL_WEIGHT_VA: u32 = 0x00ed7cdc;
        const TARGET_SLOT: u32 = 0x70;
        const TARGET_STATE_A: u32 = 0x74;
        const TARGET_STATE_B: u32 = 0x78;
        const TARGET_STATE_C: u32 = 0x7c;
        const TARGET_STATE_SMALL: u32 = 0x80;
        const MODE_STATE: u32 = 0x9c;
        const REMAINING_TIME: u32 = 0x8c;
        const INITIAL_WEIGHT: u32 = 0x90;
        const FLOAT_HELPER: u32 = 1;
        const BASE_CTOR: u32 = 2;
        const TASK_INITIALIZER: u32 = 3;
        let helper_weight = lf_checker_rt::callee_thiscall!(FLOAT_HELPER, f32, this_ptr, random_input);
        let helper_weight_bits = helper_weight.to_bits();
        let _base_result = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this_ptr, helper_weight_bits);
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(PRIMARY_VTABLE_VA));
        (this_ptr.wrapping_add(0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(SECONDARY_VTABLE_VA));
        for (source_offset, destination_offset) in [(0x00u32, 0x20u32), (0x04, 0x24), (0x08, 0x28)] {
            let value = (source_ptr.wrapping_add(source_offset) as *const u32).read_unaligned();
            (this_ptr.wrapping_add(destination_offset) as *mut u32).write_unaligned(value);
        }
        (this_ptr.wrapping_add(TARGET_SLOT) as *mut u32).write_unaligned(target);
        (this_ptr.wrapping_add(TARGET_STATE_A) as *mut u32).write_unaligned(0);
        (this_ptr.wrapping_add(TARGET_STATE_B) as *mut u32).write_unaligned(0);
        (this_ptr.wrapping_add(TARGET_STATE_C) as *mut u32).write_unaligned(0);
        (this_ptr.wrapping_add(TARGET_STATE_SMALL) as *mut u16).write_unaligned(0);
        let old_mode = (this_ptr.wrapping_add(MODE_STATE) as *const u32).read_unaligned();
        let selected_mode = ((mode & 1) | 2).wrapping_mul(2);
        (this_ptr.wrapping_add(MODE_STATE) as *mut u32).write_unaligned((old_mode & 0xffff_fff4) | selected_mode);
        (this_ptr.wrapping_add(REMAINING_TIME) as *mut u32).write_unaligned(u32::MAX);
        let initial_weight = (lf_checker_rt::relocated(INITIAL_WEIGHT_VA) as *const u32).read_unaligned();
        (this_ptr.wrapping_add(INITIAL_WEIGHT) as *mut u32).write_unaligned(initial_weight);
        let _initializer_result = lf_checker_rt::callee_thiscall!(TASK_INITIALIZER, u32, this_ptr);
        this_ptr
    }
});
