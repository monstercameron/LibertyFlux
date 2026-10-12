// original: 0x00C4E0A0 initialize_dual_vector_record_with_word_clear (proposed)
/// Initialize a task record from two three-word vectors and four scalar
/// arguments. The vectors are copied in source order to `+0x20` and `+0x30`;
/// `+0x14` is cleared, scalar words go to `+0x40`, `+0x44`, and `+0x4c`, and the byte argument
/// goes to `+0x48`. The original clears the 16-bit field at `+0x49`, including
/// the adjacent byte at `+0x4a`. Its base initializer is intercepted with
/// `this` in ECX; thiscall removes six stack words and returns `this` in EAX.
lf_checker_rt::export!(thiscall, rw_c4e0a0(
    this: u32, first_vector: u32, second_vector: u32,
    task_state: u32, flags: u32, primary_value: u32, secondary_value: u32
) -> u32 {
    unsafe {
        let type_tag = lf_checker_rt::relocated(0x00ec_a004);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        (this as *mut u32).write_unaligned(type_tag);
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(0);

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
        (this.wrapping_add(0x49) as *mut u16).write_unaligned(0);
        this
    }
});
