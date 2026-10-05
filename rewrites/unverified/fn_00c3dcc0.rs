// original: 0x00c3dcc0 train_speed_limit_for_class (proposed)
/// Return the speed limit for a vehicle class on the x87 stack.
///
/// `arg - 0x11` indexes a nine-entry map (0,2,2,2,0,0,2,2,1) selecting one
/// of three constants; any out-of-range argument (unsigned compare, so
/// negatives included) takes the default. Classes 0x11, 0x15 and 0x16
/// yield 60.0, class 0x19 yields 30.0, everything else yields 18.0.
/// The original dispatches through a byte map and jump table kept in the
/// code section; the rewrite matches directly. No calls.
///
/// Original: 0x00c3dcc0 (cdecl, one stack word; x87 double result).
lf_checker_rt::export!(cdecl, rw_00c3dcc0(arg: u32) -> f64 {
    match arg {
        0x11 | 0x15 | 0x16 => 60.0,
        0x19 => 30.0,
        _ => 18.0,
    }
});
