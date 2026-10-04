// original: 0x00b31650 task_rate_for_kind (proposed)

/// Rate constant for a task kind.
///
/// Returns 2.0 for kinds 14, 19, 21 and 28, and 5.0 for every other kind.
/// The result comes back in the floating-point register.
///
/// Original: 0x00b31650 (cdecl, one stack word; a range-checked jump table).
lf_checker_rt::export!(cdecl, rw_00b31650(kind: u32) -> f32 {
    match kind {
        14 | 19 | 21 | 28 => 2.0,
        _ => 5.0,
    }
});
