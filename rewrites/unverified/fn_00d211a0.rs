// original: 0x00D211A0 select_tick_slot_classifier
/// Clears the sign bits of two single-precision inputs and selects one of two cdecl helpers. Ordered `first > second` selects the alternate helper; equal values and unordered NaN comparisons select the default helper. Both helper calls receive `kind` followed by zero.
lf_checker_rt::export!(cdecl, rw_00d211a0(kind: u32, first_bits: u32, second_bits: u32) -> u32 {
    const SELECT_OTHER: u32 = 1;
    const SELECT_DEFAULT: u32 = 2;
    const SIGN_CLEAR_MASK: u32 = 0x7fff_ffff;
    let first = f32::from_bits(first_bits & SIGN_CLEAR_MASK);
    let second = f32::from_bits(second_bits & SIGN_CLEAR_MASK);
    let ordered_greater = !first.is_nan() && !second.is_nan() && first > second;
    if ordered_greater {
        lf_checker_rt::callee_cdecl!(SELECT_OTHER, u32, kind, 0)
    } else {
        lf_checker_rt::callee_cdecl!(SELECT_DEFAULT, u32, kind, 0)
    }
});
