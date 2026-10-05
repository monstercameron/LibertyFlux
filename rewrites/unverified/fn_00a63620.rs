// original: 0x00a63620 ped_task_range_check (proposed)

/// Range-and-aim check for a ped task: true when a point is close enough
/// and points along the task's aim axis.
///
/// `avec` is a 3-float point. `this+0x40` leads through two pointers to a
/// block holding an aim row at `+0x10` and an anchor point at `+0x30`.
/// `f3` is the range limit, or, when negative, selects the limit stored at
/// `this+0xe4` instead (a NaN `f3` keeps itself and then fails the range
/// test, returning 0).
///
/// Behaviour: `d` is the distance from the anchor to the point. Return 0
/// unless `d` is strictly below the limit. Otherwise clamp `f1` into
/// [`G_RANGE_MIN`, limit] as `x0`; the threshold `k` is `f4` when `x0` is
/// not above `d`, else the lerp `(1 - d/x0) * (f2 - f4) + f4`. Callee 0
/// (cdecl, one float arg: `d*d`, float result on the x87 stack) maps the
/// squared distance to a scale `r`; return 1 when
/// `aim.y*(r*dy) + aim.x*(r*dx) + aim.z*(r*dz)` strictly exceeds `k`, else 0
/// (NaN never exceeds).
///
/// All comparisons reproduce the original's `comiss`+`jbe` shape: an
/// unordered (NaN) comparison takes the same branch as below-or-equal, so
/// every test is written as a strict `>` with the fall-through negated.
/// Float operation order is the original's SSE order, pinned through
/// `black_box` helpers. Original: 0x00a63620 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00a63620(this: u32, avec: u32, f1: u32, f2: u32, f3: u32, f4: u32) -> u32 {
    unsafe {
        const LINK1: u32 = 0x40;
        const LINK2: u32 = 0x20;
        const AIM_ROW: u32 = 0x10;
        const ANCHOR: u32 = 0x30;
        const LIMIT_ALT: u32 = 0xe4;
        const G_RANGE_MIN: u32 = 0x00fe868c;
        const G_ONE: u32 = 0x00fe88e8;
        const CALLEE_SCALE: u32 = 0;

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

        let f3 = f32::from_bits(f3);
        let limit = if 0.0f32 > f3 { rdf(this + LIMIT_ALT) } else { f3 };
        let p1 = rd32(this + LINK1);
        let p2 = rd32(p1 + LINK2);
        let anchor = p2.wrapping_add(ANCHOR);
        let dy = sub(rdf(avec + 4), rdf(anchor + 4));
        let dx = sub(rdf(avec), rdf(anchor));
        let dz = sub(rdf(avec + 8), rdf(anchor + 8));
        let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
        let d = d2.sqrt();
        if !(limit > d) {
            return 0;
        }
        let gmin = f32::from_bits(rd32(lf_checker_rt::relocated(G_RANGE_MIN)));
        let mut x0 = f32::from_bits(f1);
        if gmin > x0 {
            x0 = gmin;
        }
        if x0 > limit {
            x0 = limit;
        }
        let f2 = f32::from_bits(f2);
        let f4 = f32::from_bits(f4);
        let k = if !(x0 > d) {
            f4
        } else {
            let r = div(d, x0);
            let one = f32::from_bits(rd32(lf_checker_rt::relocated(G_ONE)));
            add(mul(sub(one, r), sub(f2, f4)), f4)
        };
        let r: f32 = lf_checker_rt::callee_cdecl!(CALLEE_SCALE, f32, d2.to_bits());
        let aim = p2.wrapping_add(AIM_ROW);
        let t0 = mul(r, dy);
        let t2 = mul(r, dx);
        let mut dot = rdf(aim + 4);
        let t3 = mul(r, dz);
        dot = mul(dot, t0);
        dot = add(dot, mul(rdf(aim), t2));
        dot = add(dot, mul(rdf(aim + 8), t3));
        if dot > k { 1 } else { 0 }
    }
});
