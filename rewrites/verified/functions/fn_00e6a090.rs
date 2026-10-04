// original: 0x00E6A090 quotient_refresh_01 (proposed)

/// Refresh one stored single-precision quotient from its two stored operands.
///
/// Reads the numerator float `NUM` and the denominator float `DEN` from their
/// globals, divides numerator by denominator, and stores the quotient to the
/// `QUO` global. No arguments, no result: the only observable effect is the
/// four bytes written to `QUO`.
///
/// Division is IEEE-754 single precision with the original's scalar-SSE
/// semantics: a zero denominator yields a signed infinity (or NaN for 0/0)
/// without faulting, and NaN and subnormal inputs propagate bit-exactly
/// because the rewrite executes the same single `divss` in the same operand
/// order (both operands pass through `core::hint::black_box` so the compiler
/// cannot fold, commute or widen the operation).
///
/// The original merges the numerator into the low lane of XMM0 and leaves its
/// exit low lane holding the quotient; exit vector state is not a compared
/// channel (contract `ret` is `none`), so the rewrite returns a constant.
/// Original convention is a plain near `ret` with no stack arguments, which
/// the cdecl export below matches.
lf_checker_rt::export!(cdecl, rw_00e6a090() -> u32 {
    unsafe {
        /// File VA of the numerator operand global.
        const NUM: u32 = 0x01047A98;
        /// File VA of the denominator operand global.
        const DEN: u32 = 0x01047A9C;
        /// File VA of the quotient destination global.
        const QUO: u32 = 0x01682F6C;
        let num: f32 = *lf_checker_rt::global::<f32>(NUM);
        let den: f32 = *lf_checker_rt::global::<f32>(DEN);
        let quo: f32 = core::hint::black_box(num) / core::hint::black_box(den);
        *lf_checker_rt::global::<f32>(QUO) = quo;
        0
    }
});
