// original: 0x00A741B0 CTaskSimpleMoveSwim::vf13

/// Forwards the task pointer, an opaque first word, and a float bit pattern
/// to a helper, then returns true.
///
/// The float is supplied as the second stack word and reaches the helper as
/// a 32-bit stack argument. The helper is an intercepted thiscall with two
/// stack arguments. This method pops its own two stack words and returns in
/// AL.
lf_checker_rt::export!(thiscall, rw_00a741b0(this: u32, opaque: u32, amount_bits: u32) -> u8 {
    const APPLY_VALUE: u32 = 1;
    let _ = lf_checker_rt::callee_thiscall!(APPLY_VALUE, u32, this, opaque, amount_bits);
    1
});
