// original: 0x00909300 ui_coord_transform (proposed)

/// Transform one 2D input point into an output point with scaling, offset and
/// wrap-around fixups.
///
/// Arguments (cdecl, four stack words): `out_` points to two output floats,
/// `in_` points to two input floats, `a` and `b` are integer parameters. Four
/// global integers tune the mapping (`G0`..`G3` at file VAs below): the first
/// output is the first input minus `(G0*a - G1*0.5)`, the second is the
/// negated form of the second input minus `((G3-b)*G0 - G1*0.5)`. Both are
/// then scaled by `1/G0`, and each is re-based by `0.5/G3` when it lands
/// exactly on 0.0 or exactly on 1.0 (ordered float equality, so NaN takes no
/// fixup). All arithmetic is single-precision in the original's order;
/// integer products wrap.
///
/// Original: 0x00909300 (cdecl, four stack words; no return value).
lf_checker_rt::export!(cdecl, rw_00909300(out_: u32, in_: u32, a: u32, b: u32) -> u32 {
    unsafe {
        const G0: u32 = 0x0103_44DC;
        const G1: u32 = 0x0103_44E0;
        const G3B: u32 = 0x0103_44E4;
        const G3: u32 = 0x0103_44E8;
        const HALF: u32 = 0x00FE_8830;
        const ONE: u32 = 0x00FE_88E8;
        const NEG: u32 = 0x00FE_8FA0;

        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        unsafe fn gi(va: u32) -> i32 {
            unsafe { (lf_checker_rt::relocated(va) as *const i32).read() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::relocated(va) as *const u32).read()) }
        }
        #[inline(always)]
        unsafe fn rd(o: u32) -> f32 {
            unsafe { (o as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(o: u32, v: f32) {
            unsafe { (o as *mut f32).write_unaligned(v) }
        }

        let g0 = gi(G0);
        let g1 = gi(G1);
        let g3b = gi(G3B);
        let g3 = gi(G3);
        let half = gf(HALF);
        let one = gf(ONE);
        let neg = (lf_checker_rt::relocated(NEG) as *const u32).read();

        let t1 = g0.wrapping_mul(a as i32);
        let p1 = mul(g1 as f32, half);
        let mut x1 = sub(t1 as f32, p1);
        let mut x0 = sub(rd(in_), x1);
        wr(out_, x0);
        let e2 = g3b.wrapping_sub(b as i32).wrapping_mul(g0);
        let p2 = mul(g1 as f32, half);
        x1 = sub(e2 as f32, p2);
        let mut x5 = rd(out_);
        let in1 = rd(in_.wrapping_add(4));
        x0 = sub(in1, x1);
        let mut xx1 = one;
        x0 = f32::from_bits(x0.to_bits() ^ neg);
        wr(out_.wrapping_add(4), x0);
        let mut x3 = rd(out_.wrapping_add(4));
        xx1 = div(xx1, g0 as f32);
        x5 = mul(x5, xx1);
        x3 = mul(x3, xx1);
        wr(out_, x5);
        wr(out_.wrapping_add(4), x3);
        if x5 == 0.0 {
            wr(out_, add(div(half, g3 as f32), x5));
        }
        if x3 == 0.0 {
            wr(out_.wrapping_add(4), add(div(half, g3 as f32), x3));
        }
        x3 = rd(out_);
        if x3 == one {
            wr(out_, sub(x3, div(half, g3 as f32)));
        }
        xx1 = rd(out_.wrapping_add(4));
        if xx1 == one {
            wr(out_.wrapping_add(4), sub(xx1, div(half, g3 as f32)));
        }
        0
    }
});
