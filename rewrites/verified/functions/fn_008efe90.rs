// original: 0x008efe90 maybe_negate_callee
/// Call the two-argument float helper, negating while a flag is set.
///
/// Forwards both arguments to the helper (direct call, stubbed by the
/// checker) and returns its float answer, with the sign bit flipped when
/// the global switch at 0x1160C60 is nonzero.
export!(cdecl, rw_008efe90(a0: u32, a1: u32) -> f32 {
    unsafe {
        let v = callee_cdecl!(1, f32, a0, a1);
        if *global::<u32>(0x1160C60) != 0 {
            f32::from_bits(v.to_bits() ^ *global::<u32>(0xFE8FA0))
        } else {
            v
        }
    }
});
