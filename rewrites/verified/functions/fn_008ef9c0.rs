// original: 0x008ef9c0 maybe_negate_call
/// Call the one-argument helper, negating its answer while a flag is set.
///
/// Forwards `arg` to the helper (direct call, stubbed by the checker) and
/// returns its answer, negated when the global switch at 0x1160C64 is
/// nonzero.
export!(cdecl, rw_008ef9c0(arg: u32) -> u32 {
    unsafe {
        let v = callee_cdecl!(1, u32, arg);
        if *global::<u32>(0x1160C64) != 0 {
            v.wrapping_neg()
        } else {
            v
        }
    }
});
