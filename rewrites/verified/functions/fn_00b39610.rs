// original: 0x00B39610 task_vec_compute

/// Advance one task vector by a trig-weighted blend of its inputs.
///
/// `t = float(G_TICK) * K1 * K2 + G_OFF` (constant coefficients from
/// read-only data). Callee 1 and callee 2 evaluate scalar functions of `t`
/// (both take it in `xmm0` with no stack words of their own and return
/// the result in `xmm0`; the contract compares the `xmm0` argument,
/// transports it from a stack word on the rewrite side and scripts finite
/// answers).
/// With `d = (b - a) * K3` and `s = d + a` from the float arguments `a` and
/// `b`: `out = inp + ((r2*0 - r1) * s, (r1*0 + r2) * s, s * 0)`, computed in
/// the original's exact operation order (the multiply-by-zero steps decide
/// NaN propagation for non-finite scripted answers, so they are kept, not
/// folded). The scalar `d` is also stored through the fifth argument, and
/// the function returns that pointer. Cdecl, five stack words.
///
/// The original stages `t` through its own fourth-argument slot; the
/// rewrite writes the same word to the same slot so the stack comparison
/// sees it.
///
/// Original: 0x00B39610.

lf_checker_rt::export!(cdecl, rw_00B39610(inp: u32, a: u32, b: u32, out: u32, dout: u32) -> u32 {
    unsafe {
        const G_TICK: u32 = 0x016624B4;
        const G_OFF: u32 = 0x016624B8;
        const K1: u32 = 0x00FE8790;
        const K2: u32 = 0x00FE8AEC;
        const K3: u32 = 0x00FE8830;
        const F1: u32 = 1;
        const F2: u32 = 2;
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        let tick = (lf_checker_rt::global::<u32>(G_TICK) as *const u32).read_unaligned() as i32;
        let k1 = (lf_checker_rt::global::<f32>(K1) as *const f32).read_unaligned();
        let k2 = (lf_checker_rt::global::<f32>(K2) as *const f32).read_unaligned();
        let k3 = (lf_checker_rt::global::<f32>(K3) as *const f32).read_unaligned();
        let off = (lf_checker_rt::global::<f32>(G_OFF) as *const f32).read_unaligned();
        let t = add(mul(mul(tick as f32, k1), k2), off);
        // Copy the pointers first: the slot write clobbers `out` and the
        // compiler reloads it from the slot afterwards.
        let outp = out;
        let doutp = dout;
        core::hint::black_box((&out as *const u32) as *mut u32).write_unaligned(t.to_bits());
        let r1 = f32::from_bits(lf_checker_rt::callee_cdecl!(F1, u32, t.to_bits()));
        let r2 = f32::from_bits(lf_checker_rt::callee_cdecl!(F2, u32, t.to_bits()));
        let fa = f32::from_bits(a);
        let fb = f32::from_bits(b);
        let d = mul(sub(fb, fa), k3);
        let s = add(d, fa);
        let z = 0.0f32;
        let t3 = mul(sub(mul(r2, z), r1), s);
        let t4 = mul(add(mul(r1, z), r2), s);
        let t1 = mul(s, z);
        let i0 = (inp as *const f32).read_unaligned();
        let i1 = (inp.wrapping_add(4) as *const f32).read_unaligned();
        let i2 = (inp.wrapping_add(8) as *const f32).read_unaligned();
        (outp as *mut f32).write_unaligned(add(i0, t3));
        (outp.wrapping_add(4) as *mut f32).write_unaligned(add(i1, t4));
        (outp.wrapping_add(8) as *mut f32).write_unaligned(add(i2, t1));
        (doutp as *mut f32).write_unaligned(d);
        doutp
    }
});
