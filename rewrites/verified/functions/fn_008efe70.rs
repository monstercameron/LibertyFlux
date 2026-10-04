// original: 0x008efe70 maybe_negate_call2
/// Call the two-argument helper, negating its answer while a flag is set.
///
/// Forwards both arguments to the helper (direct call, stubbed by the
/// checker) and returns its answer, negated when the global switch at
/// 0x1160C60 is nonzero.
export!(cdecl, rw_008efe70(arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let v = callee_cdecl!(1, u32, arg0, arg1);
        if *global::<u32>(0x1160C60) != 0 {
            v.wrapping_neg()
        } else {
            v
        }
    }
});
