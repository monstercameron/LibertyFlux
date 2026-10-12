// original: 0x00A81E00 slide_cover_subtask_index_to_id (proposed)

/// Convert a one-based cover subtask index to its encoded identifier.
///
/// Indices 1 through 16 map to identifiers 0x114 through 0x123 by adding
/// 0x113. All other 32-bit inputs, including zero and values above the range,
/// return `u32::MAX`. The original uses an unsigned range check and pops its
/// single stack argument (`stdcall`).
lf_checker_rt::export!(stdcall, rw_00a81e00(index: u32) -> u32 {
    const FIRST_VALID_INDEX: u32 = 1;
    const LAST_VALID_INDEX: u32 = 16;
    const IDENTIFIER_BIAS: u32 = 0x113;

    if (FIRST_VALID_INDEX..=LAST_VALID_INDEX).contains(&index) {
        index + IDENTIFIER_BIAS
    } else {
        u32::MAX
    }
});
