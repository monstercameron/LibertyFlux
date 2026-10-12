// original: 0x00A81E20 slide_cover_subtask_id_to_index (proposed)

/// Convert a signed cover subtask identifier to its zero-based array index.
///
/// The `thiscall` receiver points to an object whose identifier is the signed
/// 32-bit word at `+0x18`. Identifiers 0x114 through 0x123 map to 1 through 16
/// by subtracting 0x113. Values outside that inclusive range return zero.
lf_checker_rt::export!(thiscall, rw_00a81e20(info: u32) -> u32 {
    const INFO_SUBTASK_ID: u32 = 0x18;
    const FIRST_SUBTASK_ID: i32 = 0x114;
    const LAST_SUBTASK_ID: i32 = 0x123;
    const INDEX_BIAS: u32 = 0x113;

    let subtask_id = unsafe {
        ((info.wrapping_add(INFO_SUBTASK_ID)) as *const u32).read_unaligned() as i32
    };

    if (FIRST_SUBTASK_ID..=LAST_SUBTASK_ID).contains(&subtask_id) {
        (subtask_id as u32).wrapping_sub(INDEX_BIAS)
    } else {
        0
    }
});
