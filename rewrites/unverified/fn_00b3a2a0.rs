// original: 0x00b3a2a0 task_flag_set_test (proposed)

/// Test the status word of task-table entry `index`: fetch the entry pointer
/// from the task pointer table, read the word at the status offset, and
/// return 1 when the tested status bit is set, 0 when it is clear (the
/// sibling at 0x00b3a250 returns the negated test). The result is a clean
/// 0/1. Original: 0x00b3a2a0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b3a2a0(index: u32) -> u32 {
    unsafe {
        const TASK_TABLE: u32 = 0x01295cd8;
        const STATUS_OFF: u32 = 0x120;
        const TEST_BIT: u32 = 2;
        let base = lf_checker_rt::relocated(TASK_TABLE);
        let entry = ((base + index.wrapping_mul(4)) as *const u32).read();
        let status = ((entry + STATUS_OFF) as *const u32).read();
        if status & TEST_BIT == 0 { 0 } else { 1 }
    }
});
