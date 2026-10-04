// original: 0x00d95820 seg_intersect_2d (proposed)

/// 2D segment-segment intersection test with epsilon-degenerate handling.
///
/// `a`, `b`, `c`, `d` each point to two floats (x, y). Tests whether segment
/// A->B meets segment C->D, returning 1 when the two line parameters both
/// land in [0, 1] and 0 otherwise.
///
/// Let ex = B.x-A.x, ey = B.y-A.y, fx = D.x-C.x, fy = D.y-C.y and
/// det = fy*ex - fx*ey. The entry path rejects a near-zero determinant
/// (|det| < EPS, EPS = 1e-5): a zero-area parallelogram means parallel or
/// degenerate segments. Otherwise the solver is picked by which denominator
/// is usable: |fx| < EPS solves through the x spread of A->B (needs
/// |ex| >= EPS), else |fy| < EPS solves through the y spread (needs
/// |ey| >= EPS), else the full 2x2 Cramer solve runs. Each path checks its
/// two parameters against [0, 1] with NaN failing closed (an unordered
/// compare takes the reject branch, matching comiss+jb semantics).
///
/// The original spills three SSE temporaries into its own incoming argument
/// slots; that is compiler scratch, not behaviour, so the contract runs with
/// the stack comparison off (see `narrowed`).
///
/// Original: 0x00d95820 (cdecl, four pointers, full-eax 0/1 result).
#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
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
fn fdiv(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}
#[inline(always)]
unsafe fn rd(a: u32) -> f32 {
    unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
}

const EPS: f32 = 1e-5;

lf_checker_rt::export!(cdecl, rw_00d95820(a: u32, b: u32, c: u32, d: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
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
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        const EPS: f32 = 1e-5;
        let (ax, ay) = (rd(a), rd(a + 4));
        let (bx, by) = (rd(b), rd(b + 4));
        let (cx, cy) = (rd(c), rd(c + 4));
        let (dx, dy) = (rd(d), rd(d + 4));
        let ex = fsub(bx, ax);
        let ey = fsub(by, ay);
        let fx = fsub(dx, cx);
        let fy = fsub(dy, cy);
        let det = fsub(fmul(fy, ex), fmul(fx, ey));
        // comiss+jbe: reject only on ordered |det| < EPS; NaN proceeds.
        if det.abs() < EPS {
            return 0;
        }
        if fx.abs() < EPS {
            // Degenerate in x: solve through A->B's x spread.
            // comiss+ja: reject only on ordered EPS > |ex|; NaN proceeds.
            if EPS > ex.abs() {
                return 0;
            }
            let t = fdiv(fsub(cx, ax), ex);
            let s = fdiv(fsub(fadd(fmul(t, ey), ay), cy), fy);
            if !(t >= 0.0) {
                return 0;
            }
            if !(1.0 >= t) {
                return 0;
            }
            if !(s >= 0.0) {
                return 0;
            }
            if !(1.0 >= s) {
                return 0;
            }
            return 1;
        }
        if fy.abs() < EPS {
            // Degenerate in y: solve through A->B's y spread.
            if EPS > ey.abs() {
                return 0;
            }
            let t = fdiv(fsub(cy, ay), ey);
            let s = fdiv(fsub(fadd(fmul(t, ex), ax), cx), fx);
            if !(t >= 0.0) {
                return 0;
            }
            if !(1.0 >= t) {
                return 0;
            }
            if !(s >= 0.0) {
                return 0;
            }
            if !(1.0 >= s) {
                return 0;
            }
            return 1;
        }
        // Full 2x2 solve: t along A->B, s along C->D.
        let t = fdiv(fadd(fmul(fsub(ay, cy), fx), fmul(fsub(cx, ax), fy)), det);
        let s = fdiv(fsub(fadd(fmul(t, ex), ax), cx), fx);
        if !(t >= 0.0) {
            return 0;
        }
        if !(1.0 >= t) {
            return 0;
        }
        if !(s >= 0.0) {
            return 0;
        }
        if !(1.0 >= s) {
            return 0;
        }
        1
    }
});
