// original: 0x00D7BB40 angle_wrap_lerp (proposed)

/// Wrap an angle into [-pi, pi], then blend between two values by it.
///
/// `a0` is wrapped by repeated `+-2*pi` into range while it lies outside
/// (`-pi > x` adds, `x > pi` subtracts; NaN skips both loops), folded to
/// `|x|`, and shifted by `a1` into `d0`; `m0` is `max(0, d0)` with NaN
/// passing through. With `d1 = a2 - a1`, the result is `a3` when `m0`
/// is strictly above `d1`, else `a4 - (m0 / d1) * (a4 - a3)`. Returned in
/// ST0 via `fld`. Cdecl, five float stack words. Float operation order is
/// the original's.
use lf_checker_rt::export;

export!(cdecl, rw_00d7bb40(a0b: u32, a1b: u32, a2b: u32, a3b: u32, a4b: u32) -> f32 {
    #[inline(always)]
    fn add(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) + core::hint::black_box(b)
    }
    #[inline(always)]
    fn sub(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) - core::hint::black_box(b)
    }
    #[inline(always)]
    fn mul(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) * core::hint::black_box(b)
    }
    #[inline(always)]
    fn div(a: f32, b: f32) -> f32 {
        core::hint::black_box(a) / core::hint::black_box(b)
    }
    const NEG_PI: f32 = f32::from_bits(0xc049_0fdb);
    const TWO_PI: f32 = f32::from_bits(0x40c9_0fdb);
    const POS_PI: f32 = f32::from_bits(0x4049_0fdb);
    const SIGN: u32 = 0x8000_0000;
    let mut x = f32::from_bits(a0b);
    while NEG_PI > x {
        x = add(x, TWO_PI);
    }
    while x > POS_PI {
        x = sub(x, TWO_PI);
    }
    if 0.0 > x {
        x = f32::from_bits(x.to_bits() ^ SIGN);
    }
    let a1 = f32::from_bits(a1b);
    let d0 = sub(x, a1);
    let m0 = if 0.0 > d0 { 0.0 } else { d0 };
    let a2 = f32::from_bits(a2b);
    let a3 = f32::from_bits(a3b);
    let a4 = f32::from_bits(a4b);
    let d1 = sub(a2, a1);
    let take_a3 = m0 > d1;
    let r = sub(a4, mul(div(m0, d1), sub(a4, a3)));
    if take_a3 { a3 } else { r }
});
