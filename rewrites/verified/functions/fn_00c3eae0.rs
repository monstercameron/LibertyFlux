// original: 0x00c3eae0 vec2_signed_angle (proposed)
/// Signed angle between two 2-vectors via a scripted arc-cosine helper.
///
/// `this` (ECX) and `other` (stack) each point at a block holding the
/// vector's (x, y) at `+0x04` and `+0x08`. Computes
/// `prod = (a.x^2+a.y^2)*(b.x^2+b.y^2)`; the normaliser is 0 when `prod`
/// is exactly zero (the original tests this with lahf/test/jp, which is
/// true only for equal) else `1/sqrt(prod)`. `cross = a.x*b.y-a.y*b.x`
/// picks the sign: -1.0 when `cross < 0` or NaN (the original's `jb`
/// takes the unordered case too), else +1.0. `x = dot*normaliser` is
/// clamped to [-1, +1] (NaN passes through: the upper `jbe` also takes
/// the unordered case). When `prod - 1e-12 < 0` or is NaN the result is
/// +0.0 with no call; otherwise helper id 1 is called with the clamped
/// value in XMM0 and the result is `answer*sign`, returned in ST0.
///
/// Original: 0x00c3eae0 (thiscall, one stack word, one call, f32 in ST0).
lf_checker_rt::export!(thiscall, rw_00c3eae0(this: u32, other: u32) -> f32 {
    unsafe {
        const C1: u32 = 0x3f80_0000; // 1.0 (text const, embedded)
        const C0: u32 = 0xbf80_0000; // -1.0 (text const, embedded)
        const C2: u32 = 0x2b8c_bccc; // 1e-12 (text const, embedded)
        #[inline(always)]
        unsafe fn rd(p: u32) -> f32 {
            unsafe { f32::from_bits((p as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        let one = f32::from_bits(C1);
        let neg = f32::from_bits(C0);
        let eps = f32::from_bits(C2);
        let ax = rd(this + 4);
        let ay = rd(this + 8);
        let bx = rd(other + 4);
        let by = rd(other + 8);
        let na = fadd(fmul(ax, ax), fmul(ay, ay));
        let nb = fadd(fmul(bx, bx), fmul(by, by));
        let prod = fmul(na, nb);
        let inv = if prod == 0.0 { 0.0 } else { fdiv(one, prod.sqrt()) };
        let cross = fsub(fmul(ax, by), fmul(ay, bx));
        let sign = if !(cross >= 0.0) { neg } else { one };
        let dot = fadd(fmul(ax, bx), fmul(ay, by));
        let x = fmul(dot, inv);
        let cl = if neg > x { neg } else if !(x > one) { x } else { one };
        let tail = fsub(prod, eps);
        if !(tail >= 0.0) {
            0.0
        } else {
            let f1 = f32::from_bits(lf_checker_rt::callee_cdecl!(1, u32, cl.to_bits()));
            fmul(f1, sign)
        }
    }
});
