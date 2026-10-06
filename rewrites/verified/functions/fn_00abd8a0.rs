// original: 0x00ABD8A0 vec_light_kernel_gated (proposed)

/// Gated version of the light probe: same centroid/radius front half as
/// `vec_light_kernel`, then a second helper decides whether the term applies.
///
/// Arguments (cdecl, four stack words, plain `ret`): `obj` points to the
/// object (floats at `+0x20`..`+0x28`, `+0x30`..`+0x40`, `+0x54`, `+0x5c`;
/// an integer tag at `+0x44`), `vec` to six floats at `+0x00`..`+0x18`,
/// `out4` to four output words, `out1` to one output word. Returns a float
/// on the x87 stack (`fld` of the low word, `fldz` on the early exits).
///
/// Algorithm. Front half: `half = 0.5`; `cy = half*(vec[0x00]+vec[0x10])`,
/// `cx = half*(vec[0x14]+vec[0x04])`, `cz = half*(vec[0x18]+vec[0x08])`
/// (these three pre-anchor values are also kept for the second helper call),
/// minus `obj[0x20]`, `obj[0x24]`, `obj[0x28]`; `len` is their length,
/// `inv = 1.0/len`; `out4 = (inv*cy, inv*cx, inv*cz, scratch)`,
/// `out1[0] = len`. `out4[3]` is the original's uninitialised scratch (see
/// below). Back half: `d1/d2/d3` are the halved vec spreads; the reloaded
/// `out4` row weights them, `s` is the length of the weighted triple; exit
/// with 0 when not (`obj[0x54] > 4.0`) or not (`obj[0x54]+s > len`).
/// Otherwise `w` is `obj[0x30..0x40]` dotted with `(0.2125, 0.7154, 0.0721)`;
/// helper 1 (cdecl: `len`, `s+10.0`) answers a float; the tail returns
/// `(1.0*answer)*w` when not (`len > 0`), not (`answer > 0`),
/// `obj[0x44] != 2` (equality; signedness immaterial) or not
/// (`obj[0x5c] > 0.01`). Otherwise helper 2 (thiscall: scratch struct,
/// one float word, `obj+0x20`, `obj`) runs with `0.0` when
/// `0.0 > obj[0x5c]` (unreachable: `obj[0x5c] > 0.01` held to get here),
/// else `obj[0x5c]` clamped above at `1.0`; then helper 3 (thiscall:
/// scratch struct, pointer to the three pre-anchor values, the length of
/// the halved spreads) answers a byte, and the tail returns
/// `((al != 0 ? 1.0 : 0.0)*answer)*w`. All `comiss` + branch pairs are
/// ordered comparisons with NaN taking the false/exit side, modelled as
/// `!(a > b)`. The two `sqrtps` uses only feed their low lane onward, which
/// equals `sqrtss`; upper lanes are never observed.
///
/// Edge cases: zero `len` divides to infinities; NaN anywhere in the front
/// half propagates into `len`/`inv`/outputs and takes every later exit;
/// infinities compare/arithmetic per IEEE-754, bit-exact with the original.
///
/// Fidelity note: `out4[3]` (`[a2+0xc]`) is read by the original from its
/// own stack scratch that it never wrote. Its value in the game is whatever
/// the stack held; under the checker (defined zero fill) it is `+0.0`, which
/// this rewrite emits. The comparison therefore covers that word only up to
/// the fill, disclosed in the proof record.
///
/// Original: 0x00ABD8A0 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00abd8a0(obj: u32, vec: u32, out4: u32, out1: u32) -> f32 {
    unsafe {
        const C_HALF: u32 = 0x00FE8830; // 0.5
        const C_ONE: u32 = 0x00FE88E8; // 1.0
        const C_FOUR: u32 = 0x00FE8AB8; // 4.0
        const C_TINY: u32 = 0x00FE870C; // 0.01
        const C_TEN: u32 = 0x00FE8B08; // 10.0
        const C_W0: u32 = 0x0103EF00; // 0.2125
        const C_W1: u32 = 0x0103EF04; // 0.7154
        const C_W2: u32 = 0x0103EF08; // 0.0721
        const HELPER_F32: u32 = 1;
        const HELPER_STRUCT: u32 = 2;
        const HELPER_GATE: u32 = 3;

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
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn g(v: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::global::<u32>(v)).read_unaligned()) }
        }

        let half = g(C_HALF);
        let one = g(C_ONE);

        // Centroid of the vector set, relative to the object's anchor;
        // the pre-anchor values are kept for the gate helper's struct.
        let cy_pre = mul(add(rdf(vec), rdf(vec.wrapping_add(0x10))), half);
        let cx_pre = mul(add(rdf(vec.wrapping_add(0x14)), rdf(vec.wrapping_add(0x04))), half);
        let cz_pre = mul(add(rdf(vec.wrapping_add(0x18)), rdf(vec.wrapping_add(0x08))), half);
        let cy = sub(cy_pre, rdf(obj.wrapping_add(0x20)));
        let cx = sub(cx_pre, rdf(obj.wrapping_add(0x24)));
        let cz = sub(cz_pre, rdf(obj.wrapping_add(0x28)));
        let mut x1 = mul(cx, cx);
        let mut x0 = mul(cy, cy);
        x1 = add(x1, x0);
        x0 = mul(cz, cz);
        x1 = add(x1, x0);
        let len = x1.sqrt();
        let inv = div(one, len);
        x0 = mul(inv, cy);
        x1 = inv;
        let x2b = mul(inv, cz);
        wrf(out4, x0);
        x0 = 0.0; // original reads uninitialised scratch here; fill is 0
        wrf(out4.wrapping_add(0x0c), x0);
        wrf(out4.wrapping_add(8), x2b);
        x1 = mul(x1, cx);
        wrf(out4.wrapping_add(4), x1);
        wrf(out1, len);

        // Weighted spread, then the two range gates.
        let d1 = sub(rdf(vec.wrapping_add(0x10)), rdf(vec));
        let d3 = sub(rdf(vec.wrapping_add(0x18)), rdf(vec.wrapping_add(0x08)));
        let d2 = sub(rdf(vec.wrapping_add(0x14)), rdf(vec.wrapping_add(0x04)));
        let h2 = mul(d2, half);
        let h1 = mul(d1, half);
        let mut h3 = mul(d3, half);
        let mut t2 = mul(rdf(out4), h1);
        let o4 = rdf(out4.wrapping_add(4));
        h3 = mul(h3, rdf(out4.wrapping_add(8)));
        let mut t0 = mul(o4, h2);
        t2 = mul(t2, t2);
        t0 = mul(t0, t0);
        h3 = mul(h3, h3);
        t0 = add(t0, t2);
        let o54 = rdf(obj.wrapping_add(0x54));
        let s = add(t0, h3).sqrt();
        if !(o54 > g(C_FOUR)) {
            return 0.0;
        }
        let sum = add(o54, s);
        if !(sum > len) {
            return 0.0;
        }

        // Luminance-style weighting, then the float helper call.
        x0 = rdf(obj.wrapping_add(0x30));
        x1 = rdf(obj.wrapping_add(0x34));
        x1 = mul(x1, g(C_W1));
        x0 = mul(x0, g(C_W0));
        let arg_len = len;
        let arg_sum = sum;
        x1 = add(x1, x0);
        x0 = rdf(obj.wrapping_add(0x38));
        x0 = mul(x0, g(C_W2));
        x1 = add(x1, x0);
        x1 = mul(x1, rdf(obj.wrapping_add(0x40)));
        let w = x1;
        let ans: f32 =
            lf_checker_rt::callee_cdecl!(HELPER_F32, f32, arg_len.to_bits(), arg_sum.to_bits());
        let zero = 0.0f32;
        if !(len > zero) {
            return mul(mul(one, ans), w);
        }
        if !(ans > zero) {
            return mul(mul(one, ans), w);
        }
        if rd32(obj.wrapping_add(0x44)) != 2 {
            return mul(mul(one, ans), w);
        }
        let x1b = rdf(obj.wrapping_add(0x5c));
        if !(x1b > g(C_TINY)) {
            return mul(mul(one, ans), w);
        }
        let x0sel: f32;
        if zero > x1b {
            x0sel = zero;
        } else if !(x1b > one) {
            x0sel = x1b;
        } else {
            x0sel = one;
        }
        let mut frame = [0u32; 11];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            HELPER_STRUCT,
            u32,
            frame.as_mut_ptr() as u32,
            x0sel.to_bits(),
            obj.wrapping_add(0x20),
            obj
        );

        // Gate helper over the halved spreads and the pre-anchor struct.
        let h1b = mul(d1, half);
        let h2b = mul(d2, half);
        let h3b = mul(d3, half);
        let mut q0 = mul(h2b, h2b);
        let mut q1 = mul(h1b, h1b);
        q0 = add(q0, q1);
        q1 = mul(h3b, h3b);
        q0 = add(q0, q1);
        let r = q0.sqrt();
        let mut s3 = [cy_pre, cx_pre, cz_pre];
        let gate: u32 = lf_checker_rt::callee_thiscall!(
            HELPER_GATE,
            u32,
            frame.as_mut_ptr() as u32,
            s3.as_mut_ptr() as u32,
            r.to_bits()
        );
        let x6: f32 = if gate as u8 != 0 { one } else { zero };
        mul(mul(x6, ans), w)
    }
});
