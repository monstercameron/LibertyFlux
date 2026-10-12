// original: 0x00C4DEB0 initialize_destroy_task_parameters (proposed)
/// Initialize the base task, install this task's type tag, clear its status
/// words, and copy a three-word vector from the stack pointer argument to
/// offsets `+0x20`, `+0x24`, and `+0x28` in the task object. The base-task
/// initializer is called through the checker stand-in with `this` in ECX.
/// The method returns the task pointer in EAX and removes its one stack word.
lf_checker_rt::export!(thiscall, rw_c4deb0(this: u32, vector: u32) -> u32 {
    unsafe {
        const VECTOR_DESTINATION: u32 = 0x20;
        let type_tag = lf_checker_rt::relocated(0x00ec_9efc);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
        (this as *mut u32).write_unaligned(type_tag);
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(0);
        let vector_words = vector as *const u32;
        (this.wrapping_add(VECTOR_DESTINATION) as *mut u32)
            .write_unaligned(vector_words.read_unaligned());
        (this.wrapping_add(VECTOR_DESTINATION + 4) as *mut u32)
            .write_unaligned(vector_words.add(1).read_unaligned());
        (this.wrapping_add(VECTOR_DESTINATION + 8) as *mut u32)
            .write_unaligned(vector_words.add(2).read_unaligned());
        (this.wrapping_add(0x30) as *mut u8).write(1);
        (this.wrapping_add(0x34) as *mut u32).write_unaligned(0);
        this
    }
});
