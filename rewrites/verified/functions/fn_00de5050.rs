// original: 0x00DE5050 scrolling_row_has_any_state
/// Test two row predicates in order. Return one as soon as the first
/// thiscall helper at 0x00DE51C0 or the second at 0x00DE51B0 returns a
/// nonzero low byte; otherwise return zero.
lf_checker_rt::export!(thiscall, rw_de5050(this: u32) -> u8 {
    if lf_checker_rt::callee_thiscall!(1, u8, this) != 0 {
        1
    } else {
        u8::from(lf_checker_rt::callee_thiscall!(2, u8, this) != 0)
    }
});

