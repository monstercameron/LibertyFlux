// original: 0x00cc04e0 ped_moveblend_weight_normalize (proposed)

/// Normalize one move-blend weight set so its weights sum to exactly 1.
///
/// `this` points to a weight set: a signed 32-bit entry count at `+0x00`
/// followed by records of 32 bytes starting at `+0x0c`. Each record holds
/// its blend weight as a float at `+0x00` and two payload floats at `+0x08`
/// and `+0x0c` (the remaining record bytes are untouched).
///
/// Algorithm: when the count is zero the function returns 0 at once.
/// Otherwise every weight is clamped into [0, 1] (a NaN weight is kept:
/// both ordered comparisons are false for NaN, so neither clamp stores).
/// The clamped weights are summed in index order starting from +0.0. When
/// the sum is below 1 or above ~1.05 the set is renormalized: one cdecl
/// helper (patched by the checker) maps the sum to a factor `f`, each
/// weight but the last is scaled by `f * f` in index order while the
/// scaled values are accumulated, and the last weight is set to
/// `1 - accumulated` so the total is exactly 1. A sum inside [1, ~1.05]
/// (or a NaN sum) skips renormalization. Finally every record's two
/// payload floats are each multiplied by its record's weight, in order.
///
/// The ~1.05 tolerance, the 1.0 clamp and the 1.0 constant are read from
/// the executable's read-only data; the helper takes the sum bits as its
/// single stack word and returns a float on the x87 stack.
///
/// Edge cases: count 0 returns 0 with no stores and no call. Count 1
/// renormalizes to a single weight of exactly 1. A negative count takes
/// the renormalize path with a zero sum and stores below the object; the
/// contract only feeds counts -1..8 and the rewrite mirrors the original
/// there too. Return value is the record-end pointer
/// `this + 0x0c + count * 32` for a positive count, 0 for count 0, and
/// `count * 32` (wrapping) for a negative count.
///
/// Original: 0x00cc04e0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00cc04e0(this: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0x00;
        const REC_BASE: u32 = 0x0c;
        const REC_STRIDE: u32 = 0x20;
        const PAYLOAD_A: u32 = 0x08;
        const PAYLOAD_B: u32 = 0x0c;
        const ONE: f32 = 1.0;
        const SUM_HI: f32 = f32::from_bits(0x3f86_6666); // ~1.05
        const NORM_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn rec(this: u32, i: u32) -> u32 {
            this.wrapping_add(REC_BASE).wrapping_add(i.wrapping_mul(REC_STRIDE))
        }

        let count = rd32(this.wrapping_add(COUNT_OFF)) as i32;
        if count == 0 {
            return 0;
        }
        if count > 0 {
            let n = count as u32;
            // Clamp every weight into [0, 1]; NaN survives both tests.
            for i in 0..n {
                let p = rec(this, i);
                let w = rdf(p);
                if w < 0.0 {
                    wrf(p, 0.0);
                } else if w > ONE {
                    wrf(p, ONE);
                }
            }
            // Sum in index order from +0.0 (the original sums unrolled
            // blocks of four then the remainder, still in index order).
            let mut sum = 0.0f32;
            for i in 0..n {
                sum = add(sum, rdf(rec(this, i)));
            }
            // Renormalize unless the sum sits in the [1, ~1.05] band.
            if ONE > sum || sum > SUM_HI {
                let f: f32 =
                    lf_checker_rt::callee_cdecl!(NORM_CALLEE, f32, sum.to_bits());
                let r = mul(f, f);
                let mut acc = 0.0f32;
                for i in 0..n - 1 {
                    let p = rec(this, i);
                    let t = mul(r, rdf(p));
                    wrf(p, t);
                    acc = add(acc, t);
                }
                wrf(rec(this, n - 1), sub(ONE, acc));
            }
            // Scale each record's payloads by its weight.
            for i in 0..n {
                let p = rec(this, i);
                let w = rdf(p);
                let a = rdf(p.wrapping_add(PAYLOAD_A));
                let b = rdf(p.wrapping_add(PAYLOAD_B));
                wrf(p.wrapping_add(PAYLOAD_A), mul(w, a));
                wrf(p.wrapping_add(PAYLOAD_B), mul(w, b));
            }
            return rec(this, n);
        }
        // Negative count: zero sum, still renormalized, last-weight store
        // lands below the object; the payload loop is skipped.
        let f: f32 = lf_checker_rt::callee_cdecl!(NORM_CALLEE, f32, 0.0f32.to_bits());
        let _r = mul(f, f);
        let slot = this
            .wrapping_add((count as u32).wrapping_mul(REC_STRIDE))
            .wrapping_sub(0x14);
        wrf(slot, sub(ONE, 0.0));
        (count as u32).wrapping_mul(REC_STRIDE)
    }
});
