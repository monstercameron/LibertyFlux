// original: 0x00d6ccb0 filemem_scaled_span (proposed)

/// Rescale a position within a file-memory span and add it to a base.
///
/// `this` is the file-memory context (only passed through to the two lookup
/// callees, never read here). `pos` is a position value that is also a live
/// heap pointer: both lookup callees store through it, and this function
/// uses it arithmetically against their answers.
///
/// Behaviour: call the lower-bound lookup (callee 1) with `pos` and two
/// out-words `kind` and `base` (both zeroed first), then the upper-bound
/// lookup (callee 2) with `pos` and a pointer to the incoming argument slot
/// (whose stored value the original immediately overwrites, so the write
/// through it is dead). Let `span = upper - lower` and `total = pos -
/// lower`, both wrapping unsigned. Convert both to float through a double
/// (`(double)(int32)x` plus 0.0 or 2^32 by the sign bit, narrowed to float).
/// Unless `kind` is 100, map it through the two small helpers (callees 3
/// and 4), convert that answer the same way, and multiply it by
/// `span * SCALE` (a read-only float constant). Divide `total / span`,
/// multiply by that factor, round half away from zero (compare against 0.0,
/// add or subtract 0.5, truncate toward zero; all operands are
/// non-negative so only the add side is reachable, and a NaN quotient takes
/// it too on both sides), and return `base + rounded`, wrapping.
///
/// The `kind == 100` comparison is an equality on the full 32-bit out-word
/// (signedness plays no part); the float comparison is `< 0.0`, which is
/// false for NaN exactly like the original's compare-and-branch.
///
/// Original: 0x00d6ccb0 (thiscall, ECX = this, one stack word; callee 1 takes
/// three stack words, callee 2 takes two, callees 3 and 4 are cdecl of one).
lf_checker_rt::export!(thiscall, rw_00d6ccb0(this: u32, pos: u32) -> u32 {
    unsafe {
        const KIND_PLAIN: u32 = 100;
        const SCALE_ADDR: u32 = 0x00E8AE9C;
        const HALF_ADDR: u32 = 0x00FE8830;
        const U32_BIAS_ADDR: u32 = 0x00FE8F50;
        const CALLEE_LOWER: u32 = 1;
        const CALLEE_UPPER: u32 = 2;
        const CALLEE_KINDMAP: u32 = 3;
        const CALLEE_VALUEMAP: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Unsigned int to float exactly as the original's
        /// movd/cvtdq2pd/addsd-bias/cvtpd2ps sequence: signed conversion to
        /// double, plus 0.0 or 2^32 from the read-only bias table by the
        /// sign bit (both exact), narrowed once to float (round to nearest).
        #[inline(always)]
        unsafe fn u32_to_f32(x: u32) -> f32 {
            unsafe {
                let widened = (x as i32) as f64;
                let bias_at = lf_checker_rt::relocated(U32_BIAS_ADDR) + (x >> 31) * 8;
                let bias = (bias_at as *const f64).read_unaligned();
                (core::hint::black_box(widened) + core::hint::black_box(bias)) as f32
            }
        }
        /// Bit-exact `cvttss2si` (truncate toward zero; NaN, infinities and
        /// out-of-range values give 0x80000000, unlike Rust's saturating
        /// `as` cast).
        #[inline(always)]
        fn cvttss2si_exact(x: f32) -> i32 {
            if x.is_nan() {
                return i32::MIN;
            }
            let t = x.trunc();
            if t <= -2147483648.0 || t >= 2147483648.0 {
                i32::MIN
            } else {
                t as i32
            }
        }

        let mut kind: u32 = 0;
        let mut base: u32 = 0;
        let lower: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_LOWER, u32, this, pos,
            &mut kind as *mut u32 as u32,
            &mut base as *mut u32 as u32
        );
        // The original passes a pointer to its own incoming argument slot
        // here; the callee's write through it is overwritten before any
        // read. A local holding the same value is observably identical
        // (the pointer itself is skipped, its content snapshotted).
        let mut arg_mirror: u32 = pos;
        let upper: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_UPPER, u32, this, pos,
            &mut arg_mirror as *mut u32 as u32
        );
        let span = upper.wrapping_sub(lower);
        let total = pos.wrapping_sub(lower);
        let span_f = u32_to_f32(span);
        let total_f = u32_to_f32(total);
        let scale = rdf(lf_checker_rt::relocated(SCALE_ADDR));
        let half = rdf(lf_checker_rt::relocated(HALF_ADDR));
        let factor: f32 = if kind == KIND_PLAIN {
            span_f
        } else {
            let mapped_key: u32 = lf_checker_rt::callee_cdecl!(CALLEE_KINDMAP, u32, kind);
            let mapped: u32 = lf_checker_rt::callee_cdecl!(CALLEE_VALUEMAP, u32, mapped_key);
            let weight = mul(span_f, scale);
            mul(u32_to_f32(mapped), weight)
        };
        let mut q = div(total_f, span_f);
        q = mul(q, factor);
        // `q < 0.0` is false for NaN, matching the original's
        // compare-against-zero with the branch taken on unordered.
        let adj = if q < 0.0 { sub(q, half) } else { add(q, half) };
        let rounded = cvttss2si_exact(adj);
        base.wrapping_add(rounded as u32)
    }
});
