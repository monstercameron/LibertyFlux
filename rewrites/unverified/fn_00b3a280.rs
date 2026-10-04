// original: 0x00b3a280 task_kind_in_range (proposed)

/// Read the kind word of task-table entry `index` (entry pointer from the
/// task pointer table, word at the kind offset), subtract the base kind, and
/// return 1 when the unsigned result is at most the span (i.e. the kind lies
/// in the accepted interval), 0 otherwise. The subtraction wraps, so kinds
/// below the base fail. Original: 0x00b3a280 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b3a280(index: u32) -> u32 {
    unsafe {
        const TASK_TABLE: u32 = 0x01295cd8;
        const KIND_OFF: u32 = 0x90;
        const KIND_BASE: u32 = 3;
        const KIND_SPAN: u32 = 11;
        let base = lf_checker_rt::relocated(TASK_TABLE);
        let entry = ((base + index.wrapping_mul(4)) as *const u32).read();
        let kind = ((entry + KIND_OFF) as *const u32).read();
        if kind.wrapping_sub(KIND_BASE) <= KIND_SPAN {
            1
        } else {
            0
        }
    }
});
