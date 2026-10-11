// original: 0x00D39D00 ped_task_output_clear_status_word

/// Clear the 32-bit status word at offset +0x8 in the second stack argument and return that argument pointer in EAX. The entry does not use its context or first stack argument; the observed code uses cdecl stack placement and leaves cleanup to the caller.
lf_checker_rt::export!(cdecl, rw_00d39d00(_first: u32, output: u32) -> u32 {
    unsafe {
        const STATUS_WORD: u32 = 0x08;
        (output.wrapping_add(STATUS_WORD) as *mut u32).write_unaligned(0);
        output
    }
});
