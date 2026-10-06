// original: 0x00926580 reset_sixteen_slots (proposed)

/// Reset sixteen slots by calling the slot-reset helper with 0..15.
///
/// Calls callee 1 (cdecl, one argument) sixteen times with the values 0
/// through 15 in order. Returns nothing meaningful.
///
/// Original: 0x00926580 (cdecl, no arguments). Sixteen direct calls.
lf_checker_rt::export!(cdecl, rw_00926580() -> u32 {
    for i in 0..16u32 {
        lf_checker_rt::callee_cdecl!(1, u32, i);
    }
    0
});
