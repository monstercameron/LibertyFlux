// original: 0x00cf4b20 climb_task_parity_dispatch (proposed)

/// Dispatches a task id through a two-way jump table: for ids `0x73..=0x79`
/// returns 1 when the id is odd and 0 when it is even (the table alternates
/// by offset parity). Ids outside the range clear only the low byte, keeping
/// the upper three bytes of the id-minus-base value.
///
/// Original: 0x00cf4b20 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00cf4b20(task_id: u32) -> u32 {
    const RANGE_BASE: u32 = 0x73;
    const RANGE_LAST_OFFSET: u32 = 6;
    let shifted = task_id.wrapping_sub(RANGE_BASE);
    if shifted > RANGE_LAST_OFFSET {
        shifted & 0xffff_ff00
    } else {
        // Table bytes alternate 0,1,0,1,...: even offsets take the
        // "return 1" arm, odd offsets the "return 0" arm.
        (shifted + 1) & 1
    }
});
