// original: 0x00cf4b10 climb_task_range_check (proposed)

/// Tests whether a task id falls in the small contiguous range handled by the
/// neighbouring dispatch switch (see `rw_00cf4b20`): ids `0x73..=0x7a`
/// return 1, everything else 0. Only the low byte of the return value is set;
/// the upper three bytes keep whatever the subtraction left in them.
///
/// Original: 0x00cf4b10 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00cf4b10(task_id: u32) -> u32 {
    const RANGE_BASE: u32 = 0x73;
    const RANGE_LAST_OFFSET: u32 = 7;
    let shifted = task_id.wrapping_sub(RANGE_BASE);
    (shifted & 0xffff_ff00) | ((shifted <= RANGE_LAST_OFFSET) as u32)
});
