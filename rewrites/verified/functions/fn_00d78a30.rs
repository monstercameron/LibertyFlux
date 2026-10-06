// original: 0x00D78A30 clamp_accumulate_normalize_pair (proposed)

/// Clamp a value into a symmetric range and emit a normalized pair.
///
/// Takes the helper's float `r` for `a0`. When the global counter is not
/// signed-less than 2, folds the accumulator at `a0 + 0x1084` into `*a1`
/// and clears it. Then clamps `*a1` into `[-r, +r]` (NaN-proof ordered
/// compares: NaN on either side leaves the value), and with `t = 1/r`
/// stores `1 - |c*t| * 0.7` to `*a2` and `|c*t|` to `*a3`. Returns `a3`.
/// Cdecl, four stack words. Float operation order is the original's.
use lf_checker_rt::{callee_cdecl, export, global};

const NORM: u32 = 1;

export!(cdecl, rw_00d78a30(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        const COUNTER: u32 = 0x011d6fd4;
        const ACC_OFF: u32 = 0x1084;
        const SIGN: u32 = 0x8000_0000;
        const ABS_MASK: u32 = 0x7fff_ffff;
        const ONE: f32 = f32::from_bits(0x3f80_0000);
        const FACTOR: f32 = f32::from_bits(0x3f33_3333); // 0.7
        let r = f32::from_bits(callee_cdecl!(NORM, u32, a0));
        if !(global::<i32>(COUNTER).read() < 2) {
            wrf(a1, add(rdf(a1), rdf(a0 + ACC_OFF)));
            ((a0 + ACC_OFF) as *mut u32).write_unaligned(0);
        }
        let c = rdf(a1);
        let neg = f32::from_bits(r.to_bits() ^ SIGN);
        let c = if neg > c { neg } else if c > r { r } else { c };
        wrf(a1, c);
        let t = div(ONE, r);
        let v = f32::from_bits(mul(c, t).to_bits() & ABS_MASK);
        wrf(a2, sub(ONE, mul(v, FACTOR)));
        wrf(a3, v);
        a3
    }
});
