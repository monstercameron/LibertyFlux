// original: 0x009011e0 range_curve_blend (proposed)

/// Map the global selector through a six-segment curve, blend two input
/// floats toward the global base by the curve factor, and hand the results
/// to two callees.
///
/// `pair` points to two input floats; `aux_a` and `aux_b` are opaque words
/// forwarded to the second callee. The global float `v` at `0x11609CC` is
/// looked up in a decreasing six-entry threshold table held on the stack
/// (`0x45CC6000 … 0x44228000`) against the output table at `0x1034500`: an
/// exact hit takes that entry, a value strictly above an entry linearly
/// interpolates between the neighbouring entries over the neighbouring
/// thresholds, and a value at or below every entry (or NaN) takes entry 0
/// without scaling. Every path but the last multiplies the picked value by
/// `v`, giving the factor `k`. With the base `b` from `0xFE8830` the two
/// outputs are `(x0 - b) * k + b` and `(b - x1) * k + b` (note the mirrored
/// subtraction). The first output and a scratch word go to callee 1, which
/// fills the scratch word; the scratch word plus `aux_a`/`aux_b` go to
/// callee 2. Returns 1.
///
/// Edge cases, all matching the original: when `v` exceeds the first
/// threshold the interpolation reads one stack word below the table, which
/// the original never wrote; the contract defines unwritten stack as 0 so
/// the rewrite uses 0.0 there. NaN (or any unordered result) falls through
/// every segment to the unscaled entry 0. The original's stack-cookie check
/// (callee 3) is issued the same way; the cookie arithmetic itself is
/// unobservable through the stub and is not replicated.
///
/// Original: 0x009011e0 (cdecl, three stack words; returns al = 1).
lf_checker_rt::export!(cdecl, rw_009011e0(pair: u32, aux_a: u32, aux_b: u32) -> u32 {
    unsafe {
        const T0: f32 = f32::from_bits(0x45cc_6000);
        const T1: f32 = f32::from_bits(0x45a4_7800);
        const T2: f32 = f32::from_bits(0x4579_2000);
        const T3: f32 = f32::from_bits(0x4529_5000);
        const T4: f32 = f32::from_bits(0x4492_e000);
        const T5: f32 = f32::from_bits(0x4422_8000);
        const OUT_TAB: u32 = 0x0103_4500;
        const SELECTOR: u32 = 0x0116_09cc;
        const BASE: u32 = 0x00fe_8830;
        const COOKIE: u32 = 0x0105_7fb4;

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
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

        let thresholds = [T0, T1, T2, T3, T4, T5];
        let v = rdf(lf_checker_rt::relocated(SELECTOR));
        let otab = lf_checker_rt::relocated(OUT_TAB);
        let mut k = rdf(otab);
        let mut ecx = 0u32;
        while ecx < 6 {
            let u = thresholds[ecx as usize];
            let o_cur = rdf(otab.wrapping_add(ecx.wrapping_mul(4)));
            if v == u {
                k = mul(o_cur, v);
                break;
            } else if v > u {
                let o_prev = rdf(otab.wrapping_add(ecx.wrapping_mul(4).wrapping_sub(4)));
                // Below-table read: the original's stack slot is unwritten
                // here, defined as 0 by the contract's stack fill.
                let t_prev = if ecx == 0 { 0.0f32 } else { thresholds[(ecx - 1) as usize] };
                let d1 = sub(o_prev, o_cur);
                let d2 = sub(v, u);
                let p = mul(d1, d2);
                let d3 = sub(t_prev, u);
                let q = div(p, d3);
                let r = add(q, o_cur);
                k = mul(r, v);
                break;
            }
            ecx += 1;
        }

        let b = rdf(lf_checker_rt::relocated(BASE));
        let x0 = rdf(pair);
        let x1 = rdf(pair.wrapping_add(4));
        let f1 = add(mul(sub(x0, b), k), b);
        let f2 = add(mul(sub(b, x1), k), b);
        let mut scratch: u32 = 0;
        let mut first = f1;
        lf_checker_rt::callee_cdecl!(
            1,
            u32,
            core::ptr::addr_of_mut!(scratch) as u32,
            core::ptr::addr_of_mut!(first) as u32
        );
        let _ = f2;
        lf_checker_rt::callee_cdecl!(
            2,
            u32,
            core::ptr::addr_of_mut!(scratch) as u32,
            aux_a,
            aux_b
        );
        let _ = rdf(lf_checker_rt::relocated(COOKIE));
        lf_checker_rt::callee_cdecl!(3, u32,);
        1
    }
});
