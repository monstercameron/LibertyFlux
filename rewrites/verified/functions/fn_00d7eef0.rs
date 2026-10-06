// original: 0x00D7EEF0 clamped_scaled_value (proposed)

/// Scale `a` by `b`, capping the product, and return the larger.
///
/// Computes `prod = a * b`. When `b` is not strictly above 1.0 (ordered
/// compare, NaN included) the result is `prod`. Otherwise the product is
/// capped at 25.0 from above (`min` with the original's NaN choice) and
/// the result is `max(a, capped)` (NaN in `a` loses). Returned in ST0
/// via `fld`. Cdecl, two float stack words. Float order is the original's.
use lf_checker_rt::export;

export!(cdecl, rw_00d7eef0(ab: u32, bb: u32) -> f32 {
    const UNIT: f32 = f32::from_bits(0x3f80_0000); // 1.0
    const CAP: f32 = f32::from_bits(0x41c8_0000); // 25.0
    let a = f32::from_bits(ab);
    let b = f32::from_bits(bb);
    let prod = core::hint::black_box(a) * core::hint::black_box(b);
    if !(b > UNIT) {
        return prod;
    }
    let capped = if CAP > prod { prod } else { CAP };
    if a > capped { a } else { capped }
});
