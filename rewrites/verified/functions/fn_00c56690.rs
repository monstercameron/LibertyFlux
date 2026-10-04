// original: 0x00c56690 task_check_threshold_store (proposed)

/// Store whether a task parameter meets its threshold, always reporting done.
///
/// `out` receives 1 when the parameter value is at or above threshold and 0
/// otherwise; `obj` points to the owner whose dword at `+0x40` points to the
/// parameter block holding the float at `+0x4c`. The threshold is the constant
/// 0.5. The comparison is `>=`, so an unordered (NaN) value stores 0, matching
/// the original's comiss+setae. The return value is the `out` pointer with its
/// low byte forced to 1 (the original returns with only `al` set).
///
/// Original: 0x00c56690 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00c56690(out: u32, obj: u32) -> u32 {
    unsafe {
        const OBJ_PARAM: u32 = 0x40;
        const PARAM_VALUE: u32 = 0x4c;
        const THRESHOLD: f32 = 0.5;
        let mid = ((obj + OBJ_PARAM) as *const u32).read_unaligned();
        let bits = ((mid + PARAM_VALUE) as *const u32).read_unaligned();
        let bit = u8::from(f32::from_bits(bits) >= THRESHOLD);
        (out as *mut u8).write(bit);
        (out & 0xffff_ff00) | 1
    }
});

