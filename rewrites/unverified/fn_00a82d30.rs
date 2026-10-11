// original: 0x00A82D30 task_info_progress_field_by_index

/// Select one of two task-progress fields from the entry's virtual index.
///
/// The shared lookup receives `sequence_id` and `requested_stage`; a null
/// result returns zero. The non-null entry's second virtual slot is called
/// with ECX set to the entry. Its return is one-based: values 1 through 24
/// select a byte from a fixed 24-entry selector table. Selector zero reads
/// entry `+0x14`, selector one reads `+0x1C`, and selector two returns zero.
/// Zero and values above 24 also return zero. The index comparisons are
/// unsigned, so a wrapped negative return is out of range.
lf_checker_rt::export!(thiscall, rw_00a82d30(this: u32, sequence_id: u32, requested_stage: u32) -> u32 {
    const LOOKUP_CALLEE: u32 = 1;
    const ENTRY_INDEX_SLOT: u32 = 2;
    const ENTRY_FIRST_VALUE: u32 = 0x14;
    const ENTRY_SECOND_VALUE: u32 = 0x1C;
    const LAST_VALID_ZERO_BASED_INDEX: u32 = 23;
    const SELECTOR_BY_INDEX: [u8; 24] = [
        0, 1, 1, 1, 0, 0, 2, 2, 0, 0, 2, 2,
        2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    ];

    let entry = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, this, sequence_id, requested_stage);
    if entry == 0 {
        return 0;
    }
    let one_based_index = lf_checker_rt::callee_thiscall!(ENTRY_INDEX_SLOT, u32, entry);
    let index = one_based_index.wrapping_sub(1);
    if index > LAST_VALID_ZERO_BASED_INDEX {
        return 0;
    }
    match SELECTOR_BY_INDEX[index as usize] {
        0 => unsafe { ((entry + ENTRY_FIRST_VALUE) as *const u32).read_unaligned() },
        1 => unsafe { ((entry + ENTRY_SECOND_VALUE) as *const u32).read_unaligned() },
        _ => 0,
    }
});
