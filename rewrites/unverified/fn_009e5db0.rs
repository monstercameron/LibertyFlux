// original: 0x009e5db0 classify_two_floats (proposed)

/// Classify two floats by magnitude and sign into one of 0, 1, 2, 3.
///
/// Compares `|f1|` against `|f2|` for `f1 = [p + 8]`, `f2 = [p + 0x18]`
/// (`stdcall`, one pointer word). When `|f1| <= |f2|` (or either is NaN,
/// which also takes this side, matching `comiss` + `jbe` unordered
/// behaviour) the result is 0 if `0.0 > f2` ordered, else 2. Otherwise
/// the result is 3 if `0.0 <= f1`, else 1. Negative zero compares equal
/// to positive zero on both paths.
lf_checker_rt::export!(stdcall, rw_009e5db0(p: u32) -> u32 {
    unsafe {
        const LO_OFF: u32 = 8;
        const HI_OFF: u32 = 0x18;
        let f1 = f32::from_bits(((p + LO_OFF) as *const u32).read_unaligned());
        let f2 = f32::from_bits(((p + HI_OFF) as *const u32).read_unaligned());
        // comiss(|f1|, |f2|) + jbe: taken when unordered or |f1| <= |f2|.
        let abs_le = f1.is_nan() || f2.is_nan() || f1.abs() <= f2.abs();
        if abs_le {
            // comiss(0.0, f2) + cmova: 0 only when ordered and 0.0 > f2.
            if !f2.is_nan() && 0.0f32 > f2 { 0 } else { 2 }
        } else {
            // comiss(0.0, f1) + setbe, ordered here: 3 when 0.0 <= f1.
            if 0.0f32 <= f1 { 3 } else { 1 }
        }
    }
});
