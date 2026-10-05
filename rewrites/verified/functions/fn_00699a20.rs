// original: 0x00699A20 rage::crAnimChannelQuantizeFloat::vf4

/// Samples one quantized-float channel at position `x`: rounds `x` down to
/// a segment index, then either snaps to one neighbor sample or linearly
/// interpolates between the two bracketing samples, scaled into `out`.
///
/// `x` rounds with the add-and-subtract-2^23 trick (neutralized to a signed
/// zero add for huge inputs exactly like the original's compare mask), then
/// steps down by one unless the rounded value is already below-or-equal
/// (the step uses a not-less-or-equal compare, exact down to NaN and signed
/// zero). The segment `i` truncates toward zero (out-of-range and NaN give
/// `0x80000000`) and the fraction is `x - float(i)`. A fraction above 0.999
/// snaps to sample `i+1`, a fraction at-or-below 0.001 (NaN included) snaps
/// to sample `i`, anything between lerps; indexes clamp to `count - 1`
/// with unsigned comparison (`count` at `this+0x10`). Each sample comes
/// from the extractor (callee, thiscall on `this+8`) and scales by
/// `this+0x14`/`this+0x18` as `(v as f32) * k1 + k0`; the lerp is
/// `(f1 - f0) * frac + f0` in that operation order. The original also
/// writes the fraction over its own incoming `x` slot; the rewrite keeps
/// it in a local (the value is observed through the snap/lerp branch
/// choice and the lerp output instead). Returns `out` on every path.
///
/// Original: 0x00699A20 (thiscall, two stack words, callee pops 8).
lf_checker_rt::export!(thiscall, rw_00699A20(this: u32, xbits: u32, out: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x10;
        const K1_OFF: u32 = 0x14;
        const K0_OFF: u32 = 0x18;
        const SNAP_CALLEE: u32 = 1;
        const HI_CALLEE: u32 = 2;
        const LO_CALLEE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn cvtt(v: f32) -> u32 {
            const TWO31: f32 = 2147483648.0;
            if v.is_nan() || v >= TWO31 || v < -TWO31 {
                0x8000_0000
            } else {
                (v.trunc() as i32) as u32
            }
        }
        #[inline(always)]
        unsafe fn sample(id: u32, obj: u32, idx: u32, k1: f32, k0: f32) -> f32 {
            unsafe {
                let v: u32 = lf_checker_rt::callee_thiscall!(id, u32, obj, idx);
                add(mul(v as f32, k1), k0)
            }
        }

        let x = f32::from_bits(xbits);
        let sign = xbits & 0x8000_0000;
        let absx = f32::from_bits(xbits & 0x7FFF_FFFF);
        let two23 = *lf_checker_rt::global::<f32>(0x00FE8CF8);
        let magic = f32::from_bits(if absx < two23 { 0x4B00_0000 | sign } else { sign });
        let rounded = sub(add(x, magic), magic);
        let s = f32::from_bits(sign);
        let down = sub(rounded, x);
        // NLE predicate (imm 6), bit-exact including NaN and equality.
        let step = if !(down <= s) { 1.0f32 } else { 0.0 };
        let n = sub(rounded, step);
        let i = cvtt(n);
        let frac = sub(x, (i as i32) as f32);

        let hi = *lf_checker_rt::global::<f32>(0x00FE88DC);
        let lo = *lf_checker_rt::global::<f32>(0x00FE86B4);
        let count = rd32(this + COUNT_OFF);
        let countm1 = count.wrapping_sub(1);
        let k1 = rdf(this + K1_OFF);
        let k0 = rdf(this + K0_OFF);
        let obj = this + 8;
        // NOTE: the original stores frac over its incoming x slot here.
        if !(frac > hi) {
            if !(frac > lo) {
                let mut idx = i;
                if idx > countm1 {
                    idx = countm1;
                }
                wrf(out, sample(SNAP_CALLEE, obj, idx, k1, k0));
                return out;
            }
            let mut idx1 = i.wrapping_add(1);
            if idx1 > countm1 {
                idx1 = countm1;
            }
            let v1: u32 = lf_checker_rt::callee_thiscall!(HI_CALLEE, u32, obj, idx1);
            let mut idx0 = i;
            if idx0 > countm1 {
                idx0 = countm1;
            }
            let v0: u32 = lf_checker_rt::callee_thiscall!(LO_CALLEE, u32, obj, idx0);
            let f0 = add(mul(v0 as f32, k1), k0);
            let f1 = add(mul(v1 as f32, k1), k0);
            wrf(out, add(mul(sub(f1, f0), frac), f0));
            return out;
        }
        let mut idx = i.wrapping_add(1);
        if idx > countm1 {
            idx = countm1;
        }
        wrf(out, sample(SNAP_CALLEE, obj, idx, k1, k0));
        out
    }
});
