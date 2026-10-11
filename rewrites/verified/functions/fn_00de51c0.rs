// original: 0x00DE51C0 scrolling_row_has_previous_or_next
/// Return one when either the signed previous-state predicate at 0x00DE51E0
/// or, if needed, the signed next-state predicate at 0x00DE51F0 is true. Both
/// helpers receive this in ECX and the second is called only when the first
/// returns zero.
lf_checker_rt::export!(thiscall, rw_de51c0(this: u32) -> u8 {
    if lf_checker_rt::callee_thiscall!(1, u8, this) != 0 {
        1
    } else {
        u8::from(lf_checker_rt::callee_thiscall!(2, u8, this) != 0)
    }
});

