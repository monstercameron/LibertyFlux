// original: 0x00B7E9A0 clear_task_reference_if_equal

/// Clear a task reference only when it matches the supplied task value.
///
/// `expected_task` is the first cdecl stack argument. `parent` is the second
/// stack argument and points to an object whose 32-bit task reference is at
/// byte offset `TASK_REFERENCE`. The reference is compared as an unsigned
/// 32-bit value. The function returns the value it read, including when it
/// clears the field; it does not change the caller's stack pointer.
lf_checker_rt::export!(cdecl, rw_00b7e9a0(expected_task: u32, parent: u32) -> u32 {
    const TASK_REFERENCE: u32 = 0x44;

    unsafe {
        let reference = (parent.wrapping_add(TASK_REFERENCE) as *const u32).read_unaligned();
        if reference == expected_task {
            (parent.wrapping_add(TASK_REFERENCE) as *mut u32).write_unaligned(0);
        }
        reference
    }
});
