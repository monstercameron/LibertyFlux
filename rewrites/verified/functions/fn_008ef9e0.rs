// original: 0x008ef9e0 maybe_negate_callf
/// Call the one-argument float helper, negating while a flag is set.
///
/// Forwards `arg` to the helper (direct call, stubbed by the checker) and
/// returns its float answer, with the sign bit flipped when the global
/// switch at 0x1160C64 is nonzero. The flip xors the exact mask word the
/// original uses, so signed zeros and NaNs behave identically.
export!(cdecl, rw_008ef9e0(arg: u32) -> f32 {
    unsafe {
        let v = callee_cdecl!(1, f32, arg);
        if *global::<u32>(0x1160C64) != 0 {
            f32::from_bits(v.to_bits() ^ *global::<u32>(0xFE8FA0))
        } else {
            v
        }
    }
});
