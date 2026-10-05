// original: 0x00a63990 ped_task_cone_test (proposed)

/// Point-in-volume test for a ped task: cone test, or sphere test when
/// flag bit 2 is set.
///
/// Callee 0 (thiscall on `this`, two 4-float out slots and one dword
/// slot) fills two vectors `a`, `b` and a dword `c`. Flag bit 0 of
/// `this+0x28` must be set or the result is 0.
/// - Sphere path: return whether `c*c` strictly exceeds the squared
///   distance from `a[0..3]` to the point.
/// - Cone path: callee 1 (cdecl, four float args `a0, a1, b0, b1`, float
///   result on the x87 stack) yields a base angle, shifted by pi/2 and
///   wrapped into `[0, 2pi]`; callees 2 and 3 take that angle in XMM0 and
///   return a cosine-like and sine-like factor in XMM0 (low lane). Those
///   place a rim point at distance `c*0.5` from the centre `(a0, a1)`.
///   Callee 4..7 (thiscall, frame source and destination pointers, count
///   1) normalise four axis pairs; two dot products of the results must
///   pass: the first non-negative and not above the `|b-a|` length, the
///   second (absolute) not above the centre-to-rim length; finally the
///   point's `z` must lie between `a[2]` and `b[2]`. Any failure, or any
///   NaN in a comparison, returns 0.
///
/// Every comparison reproduces the original's `comiss`+`jb` shape (an
/// unordered comparison fails like below), and the two `ucomiss`-and-flag
/// normalisation guards become `len2 == 0.0` tests (a zero length maps to
/// scale 0, anything else, NaN included, to `1/sqrt(len2)`). Float
/// operation order is the original's SSE order, pinned through `black_box`
/// helpers. Original: 0x00a63990 (thiscall, two stack words: the point and
/// an unread slot).
lf_checker_rt::export!(thiscall, rw_00a63990(this: u32, pt: u32, _a1: u32) -> u32 {
    unsafe {
        const FLAGS: u32 = 0x28;
        const FLAG_ACTIVE: u8 = 0x01;
        const FLAG_SPHERE: u8 = 0x04;
        const G_HALF: u32 = 0x00fe8830;
        const G_ONE: u32 = 0x00fe88e8;
        const G_HALF_PI: u32 = 0x00fe8978;
        const G_TAU: u32 = 0x00fe8aec;
        const G_NEG: u32 = 0x00fe8fa0;
        const G_ABS: u32 = 0x00fe8f80;
        const CALLEE_POSE: u32 = 0;
        const CALLEE_ANGLE: u32 = 1;
        const CALLEE_COS: u32 = 2;
        const CALLEE_SIN: u32 = 3;
        const CALLEE_NORM1: u32 = 4;
        const CALLEE_NORM2: u32 = 5;
        const CALLEE_NORM3: u32 = 6;
        const CALLEE_NORM4: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn glob_f(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        unsafe fn glob_w(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
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

        let flags = rd8(this + FLAGS);
        if flags & FLAG_ACTIVE == 0 {
            return 0;
        }
        let mut a = [0u32; 4];
        let mut b = [0u32; 4];
        let mut c = 0u32;
        lf_checker_rt::callee_thiscall!(
            CALLEE_POSE,
            u32,
            this,
            a.as_mut_ptr() as u32,
            b.as_mut_ptr() as u32,
            &mut c as *mut u32 as u32
        );
        let fa = [f32::from_bits(a[0]), f32::from_bits(a[1]), f32::from_bits(a[2])];
        let fb = [f32::from_bits(b[0]), f32::from_bits(b[1]), f32::from_bits(b[2])];
        let fc = f32::from_bits(c);
        if flags & FLAG_SPHERE != 0 {
            let d4 = sub(fa[1], rdf(pt + 4));
            let d0 = sub(fa[0], rdf(pt));
            let d8 = sub(fa[2], rdf(pt + 8));
            let dd = add(add(mul(d0, d0), mul(d4, d4)), mul(d8, d8));
            let cc = mul(fc, fc);
            return if cc > dd { 1 } else { 0 };
        }
        let half = glob_f(G_HALF);
        let one = glob_f(G_ONE);
        let c1: f32 = lf_checker_rt::callee_cdecl!(CALLEE_ANGLE, f32, a[0], a[1], b[0], b[1]);
        let mut ang = add(c1, glob_f(G_HALF_PI));
        let tau = glob_f(G_TAU);
        while ang < 0.0 {
            ang = add(ang, tau);
        }
        while ang > tau {
            ang = sub(ang, tau);
        }
        let radius = mul(fc, half);
        let cosv = f32::from_bits(lf_checker_rt::callee_cdecl!(CALLEE_COS, u32, ang.to_bits()));
        let px = add(mul(cosv, radius), fa[0]);
        let sinv = f32::from_bits(lf_checker_rt::callee_cdecl!(CALLEE_SIN, u32, ang.to_bits()));
        let neg = glob_w(G_NEG);
        let py = add(f32::from_bits(sinv.to_bits() ^ neg), fa[1]);
        let cx = fa[0];
        let cy = fa[1];
        let dx1 = sub(fb[0], cx);
        let dy1 = sub(fb[1], cy);
        let ex = sub(cx, px);
        let ey = sub(cy, py);
        let len1sq = add(mul(dy1, dy1), mul(dx1, dx1));
        let len2sq = add(mul(ey, ey), mul(ex, ex));
        let len1 = len1sq.sqrt();
        let len2 = len2sq.sqrt();
        let qx = sub(rdf(pt), cx);
        let qy = sub(rdf(pt + 4), cy);
        let s1 = if len1sq == 0.0 { 0.0 } else { div(one, len1sq.sqrt()) };
        let nx1 = mul(dx1, s1);
        let ny1 = mul(dy1, s1);
        let mut d1 = [0u32; 3];
        let mut d2 = [0u32; 3];
        let src1 = [nx1.to_bits(), ny1.to_bits(), a[0]];
        lf_checker_rt::callee_thiscall!(
            CALLEE_NORM1,
            u32,
            d1.as_mut_ptr() as u32,
            src1.as_ptr() as u32,
            1
        );
        let src2 = [qx.to_bits(), qy.to_bits(), nx1.to_bits()];
        lf_checker_rt::callee_thiscall!(
            CALLEE_NORM2,
            u32,
            d2.as_mut_ptr() as u32,
            src2.as_ptr() as u32,
            1
        );
        let fd1 = [f32::from_bits(d1[0]), f32::from_bits(d1[1]), f32::from_bits(d1[2])];
        let fd2 = [f32::from_bits(d2[0]), f32::from_bits(d2[1]), f32::from_bits(d2[2])];
        let dot1 = add(add(mul(fd2[0], fd1[0]), mul(fd2[1], fd1[1])), mul(fd2[2], fd1[2]));
        if !(dot1 >= 0.0) {
            return 0;
        }
        if !(len1 >= dot1) {
            return 0;
        }
        let len3sq = add(mul(ey, ey), mul(ex, ex));
        let s3 = if len3sq == 0.0 { 0.0 } else { div(one, len3sq.sqrt()) };
        let mx = mul(ex, s3);
        let my = mul(ey, s3);
        let src3 = [mx.to_bits(), my.to_bits(), qx.to_bits()];
        lf_checker_rt::callee_thiscall!(
            CALLEE_NORM3,
            u32,
            d2.as_mut_ptr() as u32,
            src3.as_ptr() as u32,
            1
        );
        let src4 = [qx.to_bits(), qy.to_bits(), nx1.to_bits()];
        lf_checker_rt::callee_thiscall!(
            CALLEE_NORM4,
            u32,
            d1.as_mut_ptr() as u32,
            src4.as_ptr() as u32,
            1
        );
        let fd1 = [f32::from_bits(d1[0]), f32::from_bits(d1[1]), f32::from_bits(d1[2])];
        let fd2 = [f32::from_bits(d2[0]), f32::from_bits(d2[1]), f32::from_bits(d2[2])];
        let dot2 = add(add(mul(fd1[0], fd2[0]), mul(fd1[1], fd2[1])), mul(fd1[2], fd2[2]));
        let absdot = f32::from_bits(dot2.to_bits() & glob_w(G_ABS));
        if !(len2 >= absdot) {
            return 0;
        }
        if !(rdf(pt + 8) >= fa[2]) {
            return 0;
        }
        if !(fb[2] >= rdf(pt + 8)) {
            return 0;
        }
        1
    }
});
