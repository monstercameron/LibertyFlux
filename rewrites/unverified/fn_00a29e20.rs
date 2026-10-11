// original: 0x00a29e20 task_record_either_predicate_is_true

/// Call the first task predicate and return one on a nonzero low byte. Only
/// when it returns zero, call the second predicate and return its low-byte
/// truth value.
lf_checker_rt::export!(thiscall, rw_00a29e20(this: u32) -> u32 {
    if lf_checker_rt::callee_thiscall!(1, u32, this) as u8 != 0 {
        1
    } else {
        u32::from(lf_checker_rt::callee_thiscall!(2, u32, this) as u8 != 0)
    }
});
