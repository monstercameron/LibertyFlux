// original: 0x00c8d7f0 merge_bounds_checked (proposed)

/// Merge a query box into an entity bounds record, subject to distance checks.
///
/// `bx` points to 8 floats: a minimum corner at `+0x00/+0x04/+0x08`, an
/// unused word at `+0x0c`, a maximum corner at `+0x10/+0x14/+0x18` and an
/// unused word at `+0x1c`. `delta` and `center` point to 3 floats each; the
/// query box is `[delta-center, delta+center]` per component. `extent`
/// points to 3 floats used by the fallback check.
///
/// The merged minimum is the per-component minimum of the stored minimum
/// and `delta-center`, the merged maximum the per-component maximum of the
/// stored maximum and `delta+center` (each comparison is an ordered
/// greater-than: an unordered/NaN operand keeps the old stored value on the
/// minimum side and the stored value on the maximum side, matching the
/// original's `comiss`+`ja` selection exactly).
///
/// Acceptance: let `q0` be the squared distance from the stored minimum to
/// the merged minimum. If `LIMIT_LO (1.0)` is not above `q0` the function
/// goes to the fallback check; otherwise `q1`, the squared distance from
/// the stored maximum to the merged maximum, is computed and values with
/// `LIMIT_LO > q1` are accepted at once. The fallback accepts unless a
/// merged span exceeds twice the matching `extent` component. An accepted
/// record stores the merged corners; a rejected one is left untouched and
/// 0 is returned.
///
/// The two unused words (`+0x0c`, `+0x1c`) receive four bytes of whatever
/// happens to lie in the caller's stack frame (the original reads a frame
/// slot it never wrote). Under the checker this is the defined stack fill;
/// see the `narrowed` note on this function's result.
///
/// Original: 0x00c8d7f0 (thiscall, three stack words). Returns 1/0 in `al`.
lf_checker_rt::export!(thiscall, rw_00c8d7f0(bx: u32, delta: u32, center: u32, extent: u32) -> u32 {
    unsafe {
        const LIMIT_LO: u32 = 0x00fe88e8;
        const TWICE: u32 = 0x00fe8a24;
        /// Value the checker fills uninitialized stack with; stands in for
        /// the caller-frame bytes the original copies into the pad words.
        const STACK_FILL: u32 = 0;
        const BX_MIN0: u32 = 0x00;
        const BX_MIN1: u32 = 0x04;
        const BX_MIN2: u32 = 0x08;
        const BX_PAD0: u32 = 0x0c;
        const BX_MAX0: u32 = 0x10;
        const BX_MAX1: u32 = 0x14;
        const BX_MAX2: u32 = 0x18;
        const BX_PAD1: u32 = 0x1c;

        #[inline(always)]
        unsafe fn rd(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wr(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
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
        /// Ordered minimum selection: `if c > t { t } else { c }`.
        #[inline(always)]
        fn pick_min(c: f32, t: f32) -> f32 {
            if c > t { t } else { c }
        }
        /// Ordered maximum selection: `if s > c { s } else { c }`.
        #[inline(always)]
        fn pick_max(s: f32, c: f32) -> f32 {
            if s > c { s } else { c }
        }

        let (d0, d1, d2) = (rd(delta), rd(delta + 4), rd(delta + 8));
        let (a0, a1, a2) = (rd(center), rd(center + 4), rd(center + 8));
        let (c0, c1, c2) = (rd(bx + BX_MIN0), rd(bx + BX_MIN1), rd(bx + BX_MIN2));
        let (e0, e1, e2) = (rd(bx + BX_MAX0), rd(bx + BX_MAX1), rd(bx + BX_MAX2));
        let (t0, t1, t2) = (sub(d0, a0), sub(d1, a1), sub(d2, a2));
        let (s0, s1, s2) = (add(a0, d0), add(a1, d1), add(a2, d2));
        let (m0, m1, m2) = (pick_min(c0, t0), pick_min(c1, t1), pick_min(c2, t2));
        let (x0, x1, x2) = (pick_max(s0, e0), pick_max(s1, e1), pick_max(s2, e2));
        let lim: f32 = rd(lf_checker_rt::relocated(LIMIT_LO));
        let (dx0, dx1, dx2) = (sub(c0, m0), sub(c1, m1), sub(c2, m2));
        let q0 = add(add(mul(dx1, dx1), mul(dx0, dx0)), mul(dx2, dx2));
        let direct = lim > q0
            && {
                let (ex0, ex1, ex2) = (sub(e0, x0), sub(e1, x1), sub(e2, x2));
                let q1 = add(add(mul(ex1, ex1), mul(ex0, ex0)), mul(ex2, ex2));
                lim > q1
            };
        let ok = if direct {
            true
        } else {
            let k2: f32 = rd(lf_checker_rt::relocated(TWICE));
            let (f0, f1, f2) = (rd(extent), rd(extent + 4), rd(extent + 8));
            !(sub(x0, m0) > mul(f0, k2))
                && !(sub(x1, m1) > mul(f1, k2))
                && !(sub(x2, m2) > mul(f2, k2))
        };
        if ok {
            // Store order is the original's.
            wr(bx + BX_MIN0, m0);
            wr(bx + BX_MIN2, m2);
            ((bx + BX_PAD0) as *mut u32).write_unaligned(STACK_FILL);
            wr(bx + BX_MIN1, m1);
            wr(bx + BX_MAX0, x0);
            wr(bx + BX_MAX1, x1);
            wr(bx + BX_MAX2, x2);
            ((bx + BX_PAD1) as *mut u32).write_unaligned(STACK_FILL);
            1
        } else {
            0
        }
    }
});
