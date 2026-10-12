// original: 0x00C4DF60 initialize_dual_vector_task_record (proposed)
/// Initialize a task record from two three-word vectors and four scalar
/// arguments. The two vectors are copied in source order to `+0x20` and
/// `+0x30`; the scalar words go to `+0x40`, `+0x44`, and `+0x4c`, the byte
/// argument goes to `+0x48`, and `+0x49` is cleared. The base initializer is
/// called through a checker stand-in with `this` in ECX. Thiscall cleans all
/// six argument words and returns the task pointer in EAX.
lf_checker_rt::export!(thiscall, rw_c4df60(
    this: u32, first_vector: u32, second_vector: u32,
    task_state: u32, flags: u32, primary_value: u32, secondary_value: u32
) -> u32 {
    unsafe {
        const TYPE_TAG_VA: u32 = 0x00ec_9f54;
        let type_tag = lf_checker_rt::relocated(TYPE_TAG_VA);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        (this as *mut u32).write_unaligned(type_tag);

        let first_words = first_vector as *const u32;
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(first_words.read_unaligned());
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(first_words.add(1).read_unaligned());
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(first_words.add(2).read_unaligned());

        let second_words = second_vector as *const u32;
        (this.wrapping_add(0x30) as *mut u32).write_unaligned(second_words.read_unaligned());
        (this.wrapping_add(0x34) as *mut u32).write_unaligned(second_words.add(1).read_unaligned());
        (this.wrapping_add(0x38) as *mut u32).write_unaligned(second_words.add(2).read_unaligned());

        (this.wrapping_add(0x40) as *mut u32).write_unaligned(primary_value);
        (this.wrapping_add(0x44) as *mut u32).write_unaligned(secondary_value);
        (this.wrapping_add(0x48) as *mut u8).write(flags as u8);
        (this.wrapping_add(0x4c) as *mut u32).write_unaligned(task_state);
        (this.wrapping_add(0x49) as *mut u8).write(0);
        this
    }
});
