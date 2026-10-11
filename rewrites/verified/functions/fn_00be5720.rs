// original: 0x00BE5720 task_type_tag_or_invalid

/// Select one of four task type tags; other unsigned selectors produce the
/// invalid sentinel. The original uses an internal jump table, with no calls
/// or memory writes.
lf_checker_rt::export!(cdecl, rw_00be5720(selector: u32) -> u32 {
    const TASK_TAGS: [u32; 4] = [0x9b, 0x9c, 0xa0, 0x9e];
    TASK_TAGS.get(selector as usize).copied().unwrap_or(u32::MAX)
});
