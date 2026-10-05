// original: 0x00ae4fe0 transformed_box_overlap_score (proposed)

/// Transform a derived box by the matrix at `this` and score its overlap.
///
/// `p0` and `p1` each point to three floats (x, y, z). The function forms the
/// per-component sums `S = p1 + p0` and differences `D = p0 - p1`, builds the
/// eight corners of the box from `D` to `S` (bit `k` of the corner index
/// picks the sum for component `k`, exactly like `aabb_corners_homogeneous`,
/// with w = 1.0), and multiplies each corner by the 4x4 matrix of 16 floats
/// at `this+0x00-0x3c` (rows 16 bytes apart; output component `j` is
/// `M[0][j]*x + M[1][j]*y + M[2][j]*z + M[3][j]*w`, accumulated left to right).
/// The per-component minima and maxima over the eight corners start from
/// +FLT_MAX / -FLT_MAX (tables at `SEED_MIN` / `SEED_MAX`); a corner takes
/// the min slot unless it is strictly greater, and the max slot unless the
/// max is strictly greater, so a NaN component lands in both.
///
/// Let `ex` / `ey` be the x / y extent (max minus min). Four gate groups are
/// formed, each the AND of two scaled-extent tests against the threshold 2.0
/// at `THRESH`: group 0 is `([this+0xa0]*ey > 2) & ([this+0x90]*ex > 2)` and
/// groups 1-3 pair `+0xa4/+0x94`, `+0xa8/+0x98`, `+0xac/+0x9c` the same way.
/// If every group is zero the result is 0.
///
/// Otherwise four overlap tests run against the box at `this+0x50-0x8c`:
/// test 0 is `(s70 > minx) & (s80 > miny) & (maxx > s50) & (maxy > s60)` and
/// tests 1-3 shift each bound by one slot (`74/84/54/64`, `78/88/58/68`,
/// `7c/8c/5c/6c`). Each test is masked by its gate group being nonzero and
/// the result is `t0 + 2*t1 + 4*t2 + 8*t3` (weights at `W1/W2/W3`), formed as
/// `((t1*2 + t0) + t2*4) + t3*8` and truncated to an integer. Every comparison
/// is a strict `>` (unordered/NaN counts as false); the only NaN-sensitive
/// spots are the min/max takes, which keep the NaN.
///
/// Both return paths call the CRT security-cookie check (intercepted callee
/// 1, no arguments, registers preserved); the rewrite calls its stub.
///
/// Original: 0x00ae4fe0 (thiscall, `this` in ecx, two stack words, callee
/// pops 8). Float operation order below is the original's, pinned through
/// `black_box` so the compiler cannot commute operands.
lf_checker_rt::export!(thiscall, rw_00ae4fe0(this: u32, p0: u32, p1: u32) -> u32 {
    unsafe {
        const SEED_MIN: u32 = 0x00ea77e0;
        const SEED_MAX: u32 = 0x00ea77f0;
        const THRESH: u32 = 0x0103f728;
        const W1: u32 = 0x00fe8a24;
        const W2: u32 = 0x00fe8ab8;
        const W3: u32 = 0x00fe8afc;
        const MAT: u32 = 0x00;
        const ONE: f32 = 1.0;
        const COOKIE_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn glob_f(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned()) }
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

        let (p0x, p0y, p0z) = (rdf(p0), rdf(p0 + 4), rdf(p0 + 8));
        let (p1x, p1y, p1z) = (rdf(p1), rdf(p1 + 4), rdf(p1 + 8));
        let (sx, sy, sz) = (add(p1x, p0x), add(p1y, p0y), add(p1z, p0z));
        let (dx, dy, dz) = (sub(p0x, p1x), sub(p0y, p1y), sub(p0z, p1z));
        let (mut minx, mut miny, mut minz, mut minw) =
            (glob_f(SEED_MIN), glob_f(SEED_MIN + 4), glob_f(SEED_MIN + 8), glob_f(SEED_MIN + 12));
        let (mut maxx, mut maxy, mut maxz, mut maxw) =
            (glob_f(SEED_MAX), glob_f(SEED_MAX + 4), glob_f(SEED_MAX + 8), glob_f(SEED_MAX + 12));
        let mut i = 0u32;
        while i < 8 {
            let x = if i & 1 == 1 { sx } else { dx };
            let y = if i & 2 == 2 { sy } else { dy };
            let z = if i & 4 == 4 { sz } else { dz };
            let m = |o: u32| rdf(this + MAT + o);
            let ox = add(add(add(mul(m(0), x), mul(m(0x10), y)), mul(m(0x20), z)), mul(m(0x30), ONE));
            let oy = add(add(add(mul(m(4), x), mul(m(0x14), y)), mul(m(0x24), z)), mul(m(0x34), ONE));
            let oz = add(add(add(mul(m(8), x), mul(m(0x18), y)), mul(m(0x28), z)), mul(m(0x38), ONE));
            let ow = add(add(add(mul(m(0xc), x), mul(m(0x1c), y)), mul(m(0x2c), z)), mul(m(0x3c), ONE));
            if !(ox > minx) { minx = ox; }
            if !(oy > miny) { miny = oy; }
            if !(oz > minz) { minz = oz; }
            if !(ow > minw) { minw = ow; }
            if !(maxx > ox) { maxx = ox; }
            if !(maxy > oy) { maxy = oy; }
            if !(maxz > oz) { maxz = oz; }
            if !(maxw > ow) { maxw = ow; }
            i += 1;
        }
        let _ = (minz, minw, maxz, maxw);
        let ex = sub(maxx, minx);
        let ey = sub(maxy, miny);
        let t = glob_f(THRESH);
        let g = |ys: u32, xs: u32| -> u8 { ((mul(rdf(this + ys), ey) > t) & (mul(rdf(this + xs), ex) > t)) as u8 };
        let (g0, g1, g2, g3) = (g(0xa0, 0x90), g(0xa4, 0x94), g(0xa8, 0x98), g(0xac, 0x9c));
        if g0 == 0 && g1 == 0 && g2 == 0 && g3 == 0 {
            lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return 0;
        }
        let b = |addr: u32, v: f32| -> u8 { (rdf(this + addr) > v) as u8 };
        let c = |v: f32, addr: u32| -> u8 { (v > rdf(this + addr)) as u8 };
        let e0 = b(0x70, minx) & b(0x80, miny) & c(maxx, 0x50) & c(maxy, 0x60);
        let e1 = b(0x74, minx) & b(0x84, miny) & c(maxx, 0x54) & c(maxy, 0x64);
        let e2 = b(0x78, minx) & b(0x88, miny) & c(maxx, 0x58) & c(maxy, 0x68);
        let e3 = b(0x7c, minx) & b(0x8c, miny) & c(maxx, 0x5c) & c(maxy, 0x6c);
        let (f0, f1, f2, f3) =
            (e0 & (g0 != 0) as u8, e1 & (g1 != 0) as u8, e2 & (g2 != 0) as u8, e3 & (g3 != 0) as u8);
        // Exact small-integer arithmetic in float form; truncation is exact.
        let score = add(add(add(mul(f1 as f32, glob_f(W1)), f0 as f32), mul(f2 as f32, glob_f(W2))), mul(f3 as f32, glob_f(W3)));
        lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        score as i32 as u32
    }
});
