// original: 0x00dcf080 csv_rect_hit_test (proposed)

/// Hit test: true when the scaled counter point falls inside the adjusted
/// rectangle. All bounds are clamped to [0, bound] first.
///
/// `this` holds four floats: base x at `+0x8`, base y at `+0xc`, width at
/// `+0x10`, height at `+0x14`. The four stack words are float adjustments:
/// left = clamp(x - a0), top = clamp(y - a2), right = clamp((w + x) + a1),
/// bottom = clamp((h + y) + a3), evaluated in that operand order. The point
/// comes from two globals: px = clamp(float(cnt0) * scl0) with the i32 at
/// 0x18b7a8c scaled by the float at 0x17accf0, and py likewise from
/// 0x18b7a80 / 0x17acce8. The clamp bound is the float global at 0xfe88e8
/// (1.0 in the pristine image).
///
/// Each clamp is `if v < 0 { 0 } if v > bound { bound }`, which reproduces
/// the original's comiss+jbe pairs exactly: an unordered (NaN) compare
/// takes the jbe branch, i.e. NaN passes through. The four hit compares are
/// comiss+jb, which FAIL on unordered (NaN sets CF, so jb is taken): the
/// test passes only when py >= left, right >= py, px >= top and bottom >= px
/// all hold as ORDERED compares.
///
/// Original: 0x00DCF080 (thiscall, four stack words, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf080(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const RX: u32 = 8;
        const RY: u32 = 0xc;
        const RW: u32 = 0x10;
        const RH: u32 = 0x14;
        /// Clamp bound (1.0 in the pristine image, varied by the contract).
        const BOUND: u32 = 0xfe88e8;
        const CNT0: u32 = 0x18b7a8c;
        const SCL0: u32 = 0x17accf0;
        const CNT1: u32 = 0x18b7a80;
        const SCL1: u32 = 0x17acce8;

        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        /// Clamp to [0, bound] as the comiss+jbe pairs do: an unordered
        /// (NaN) compare takes the branch, i.e. the value passes through.
        #[inline(always)]
        fn clamp01(v: f32, bound: f32) -> f32 {
            let mut x = v;
            if x < 0.0 {
                x = 0.0;
            }
            if x > bound {
                x = bound;
            }
            x
        }

        let bound = rdf(lf_checker_rt::relocated(BOUND));
        let x = rdf(this + RX);
        let y = rdf(this + RY);
        let left = clamp01(sub(x, f32::from_bits(a0)), bound);
        let top = clamp01(sub(y, f32::from_bits(a2)), bound);
        let right = clamp01(add(add(rdf(this + RW), x), f32::from_bits(a1)), bound);
        let bottom = clamp01(add(add(rdf(this + RH), y), f32::from_bits(a3)), bound);
        let i0 = (lf_checker_rt::global::<i32>(CNT0) as *const i32).read_unaligned();
        let m0 = rdf(lf_checker_rt::relocated(SCL0));
        let px = clamp01(mul(i0 as f32, m0), bound);
        let i1 = (lf_checker_rt::global::<i32>(CNT1) as *const i32).read_unaligned();
        let m1 = rdf(lf_checker_rt::relocated(SCL1));
        let py = clamp01(mul(i1 as f32, m1), bound);
        // Each comiss+jb fails on ordered less AND on unordered (NaN sets
        // CF, so jb is taken): fail unless ordered greater-or-equal.
        if !(py >= left) {
            return 0;
        }
        if !(right >= py) {
            return 0;
        }
        if !(px >= top) {
            return 0;
        }
        if !(bottom >= px) {
            return 0;
        }
        1
    }
});
