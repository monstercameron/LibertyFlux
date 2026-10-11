// original: 0x00dc84b0 task_ctor_fixed_weight_position

/// Construct the fixed-weight variant of the task, copying a three-word
/// position and storing its target pointer. The first stack word is forwarded
/// unchanged to the base constructor; the final byte selects one mode bit
/// while clearing the other constructor-owned bits at `+0x9c`. The task
/// initializer then runs. This thiscall routine cleans four stack words.
lf_checker_rt::export!(thiscall, rw_00dc84b0(this_ptr: u32, weight_bits: u32, source_ptr: u32, target: u32, mode: u32) -> u32 {
    unsafe {
        const PRIMARY_VTABLE_VA: u32 = 0x00ef4e1c;
        const SECONDARY_VTABLE_VA: u32 = 0x00ef4e74;
        const BASE_CTOR: u32 = 1;
        const TASK_INITIALIZER: u32 = 2;
        let _base_result = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this_ptr, weight_bits);
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(PRIMARY_VTABLE_VA));
        (this_ptr.wrapping_add(0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(SECONDARY_VTABLE_VA));
        for (source_offset, destination_offset) in [(0x00u32, 0x20u32), (0x04, 0x24), (0x08, 0x28)] {
            let value = (source_ptr.wrapping_add(source_offset) as *const u32).read_unaligned();
            (this_ptr.wrapping_add(destination_offset) as *mut u32).write_unaligned(value);
        }
        (this_ptr.wrapping_add(0x70) as *mut u32).write_unaligned(target);
        for offset in [0x74u32, 0x78, 0x7c] {
            (this_ptr.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        (this_ptr.wrapping_add(0x80) as *mut u16).write_unaligned(0);
        let old_mode = (this_ptr.wrapping_add(0x9c) as *const u32).read_unaligned();
        let selected_mode = ((mode & 1) | 2).wrapping_mul(2);
        (this_ptr.wrapping_add(0x9c) as *mut u32).write_unaligned((old_mode & 0xffff_fff4) | selected_mode);
        (this_ptr.wrapping_add(0x8c) as *mut u32).write_unaligned(u32::MAX);
        (this_ptr.wrapping_add(0x90) as *mut u32).write_unaligned(0x3e80_0000);
        let _initializer_result = lf_checker_rt::callee_thiscall!(TASK_INITIALIZER, u32, this_ptr);
        this_ptr
    }
});
