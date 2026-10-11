// original: 0x00DC79B0 task_record_set_default_type

/// Forward to the task-type setter with the fixed type value seven.
///
/// This thiscall wrapper replaces its incoming stack argument with seven and
/// tail-jumps to the setter. The setter receives the same task-record pointer
/// in ECX, receives seven as its sole stack argument, returns that value, and
/// removes the wrapper's one incoming stack word.
lf_checker_rt::export!(thiscall, rw_00DC79B0(record: u32, _ignored_type: u32) -> u32 {
    const DEFAULT_TYPE: u32 = 7;
    lf_checker_rt::callee_thiscall!(1, u32, record, DEFAULT_TYPE)
});
