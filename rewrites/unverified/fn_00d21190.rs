// original: 0x00D21190 classify_time_slot_default
/// Forwards `value` and a zero second input to the time-slot classifier, then returns its result. The function takes one cdecl stack word and leaves caller cleanup to its caller.
lf_checker_rt::export!(cdecl, rw_00d21190(value: u32) -> u32 {
    const CLASSIFY_TIME: u32 = 1;
    lf_checker_rt::callee_cdecl!(CLASSIFY_TIME, u32, value, 0u32)
});
