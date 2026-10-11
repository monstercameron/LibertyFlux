// original: 0x00dc8550 task_ctor_two_positions_and_scalar

/// Construct a task from a raw word, a primary three-word position, a raw
/// float word and a second three-word position. The primary vector is copied
/// to `+0x20`, the float word to `+0x30`, and the second vector to `+0x40`.
/// The vtable value uses the relocated image base. This thiscall routine
/// cleans four stack words and returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00dc8550(this_ptr: u32, metadata: u32, primary_ptr: u32, scalar_bits: u32, secondary_ptr: u32) -> u32 {
    unsafe {
        const CLASS_VTABLE_VA: u32 = 0x00ef4dc4;
        const METADATA: u32 = 0x14;
        const PRIMARY_POSITION: u32 = 0x20;
        const SCALAR_VALUE: u32 = 0x30;
        const SECONDARY_POSITION: u32 = 0x40;
        const BASE_CTOR: u32 = 1;
        let _base_result = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this_ptr);
        (this_ptr.wrapping_add(METADATA) as *mut u32).write_unaligned(metadata);
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(CLASS_VTABLE_VA));
        for (source_offset, destination_offset) in [(0x00u32, 0x00u32), (0x04, 0x04), (0x08, 0x08)] {
            let value = (primary_ptr.wrapping_add(source_offset) as *const u32).read_unaligned();
            (this_ptr.wrapping_add(PRIMARY_POSITION + destination_offset) as *mut u32).write_unaligned(value);
        }
        (this_ptr.wrapping_add(SCALAR_VALUE) as *mut u32).write_unaligned(scalar_bits);
        for (source_offset, destination_offset) in [(0x00u32, 0x00u32), (0x04, 0x04), (0x08, 0x08)] {
            let value = (secondary_ptr.wrapping_add(source_offset) as *const u32).read_unaligned();
            (this_ptr.wrapping_add(SECONDARY_POSITION + destination_offset) as *mut u32).write_unaligned(value);
        }
        this_ptr
    }
});
