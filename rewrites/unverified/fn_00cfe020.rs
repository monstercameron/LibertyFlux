// original: 0x00cfe020 ped_task_decision_scan

/// Scan a 16-entry crowd array, steer toward its centroid, normalise.
///
/// `this` is the task object, `arg0` the subject record and `out` a 16-byte
/// record the function fills. Returns al = 1 always.
///
/// Behaviour: the scan reads 16 entries from the array at
/// `[arg0+0x224]+0x168`. A null entry, the subject itself, the record at
/// `[this+0x18]` and any entry the probe callee rejects are stored as null;
/// any other entry whose squared distance from the subject geometry is
/// strictly below the bound is stored, and the largest stored distance and
/// the accept count are tracked. With no acceptances the source triple at
/// `[[this+0x18]+0x20]+0x30` is copied to `out`; otherwise the accepted
/// entries' geometries plus the subject's own (the averaging window starts
/// one slot below the stored array, so the subject is always included and
/// the last stored entry never is) are averaged, a lazily allocated global
/// picks one of two steering computations through the query callee, and the
/// result lands in `out` with a zero tag word. The tail always runs: it
/// scales the `out`-minus-base delta to a unit vector times its constant
/// (a zero delta keeps `out` at the base) and stores it back, again with a
/// zero tag word. Both tag words read unwritten stack on the original; the
/// contract defines that fill as zero.
///
/// Original: 0x00cfe020 (thiscall, two stack words; returns al).
#[allow(clippy::all)]
#[allow(unsafe_code)]
unsafe fn run_00cfe020(this: u32, arg0: u32, out: u32, mutate_dispatch: bool) -> u32 {
    unsafe {
        const PROBE_CALLEE: u32 = 1;
        const ALLOC_CALLEE: u32 = 2;
        const INIT_CALLEE: u32 = 3;
        const QUERY_CALLEE: u32 = 4;
        const SQRT_CALLEE: u32 = 5;
        const G_BOUND: u32 = 0xfe8bb0;
        const G_ONE: u32 = 0xfe88e8;
        const G_SHARED: u32 = 0x167e3b4;
        const G_K3: u32 = 0xfe8a94;
        const G_K005: u32 = 0xfe876c;
        const G_K20: u32 = 0xfe8b38;
        const G_K5: u32 = 0xfe8ad8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(g32(va)) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn sqrt(a: f32) -> f32 {
            core::hint::black_box(a).sqrt()
        }

        let a = rd32(this.wrapping_add(0x18));
        let b = rd32(a.wrapping_add(0x20));
        let src = b.wrapping_add(0x30);
        let c = rd32(arg0.wrapping_add(0x224));
        let d = rd32(arg0.wrapping_add(0x20));

        let mut stored = [0u32; 16];
        let mut best = 0.0f32;
        let mut count = 1u32;
        let mut k = 0usize;
        while k < 16 {
            let e = rd32(c.wrapping_add(0x168).wrapping_add((k as u32) * 4));
            if e != 0 {
                let f = rd32(e.wrapping_add(0x20));
                let dx = sub(rdf(f.wrapping_add(0x30)), rdf(d.wrapping_add(0x30)));
                let dy = sub(rdf(f.wrapping_add(0x34)), rdf(d.wrapping_add(0x34)));
                let dz = sub(rdf(f.wrapping_add(0x38)), rdf(d.wrapping_add(0x38)));
                // Operand order is the original's: (dy*dy + dx*dx) + dz*dz.
                let dist = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                if e != arg0 && e != a {
                    let ok: u32 = lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, c, e);
                    if ok as u8 != 0 && gf(G_BOUND) > dist {
                        stored[k] = e;
                        if dist > best {
                            best = dist;
                        }
                        count += 1;
                    }
                }
            }
            k += 1;
        }

        let (ox, oy, oz);
        if count == 1 {
            ox = rdf(src);
            oy = rdf(src.wrapping_add(4));
            oz = rdf(src.wrapping_add(8));
        } else {
            // The averaging window starts one slot below the stored array.
            let mut sx = 0.0f32;
            let mut sy = 0.0f32;
            let mut sz = 0.0f32;
            let mut j = 0usize;
            while j < 16 {
                let e = if j == 0 { arg0 } else { stored[j - 1] };
                if e != 0 {
                    let f = rd32(e.wrapping_add(0x20));
                    sx = add(sx, rdf(f.wrapping_add(0x30)));
                    sy = add(sy, rdf(f.wrapping_add(0x34)));
                    sz = add(sz, rdf(f.wrapping_add(0x38)));
                }
                j += 1;
            }
            let inv = div(1.0, count as f32);
            let ax = mul(sx, inv);
            let ay = mul(sy, inv);
            let az = mul(sz, inv);
            let mut g = g32(G_SHARED);
            if g == 0 {
                let p: u32 = lf_checker_rt::callee_cdecl!(ALLOC_CALLEE, u32, 0x20020u32);
                g = if p == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(INIT_CALLEE, u32, p)
                };
                lf_checker_rt::global::<u32>(G_SHARED).write(g);
            }
            let r: u32 = lf_checker_rt::callee_thiscall!(QUERY_CALLEE, u32, g, rd32(c.wrapping_add(0xd8)));
            let flag = (rd32(r.wrapping_add(0x8f4)) >> 13) & 3;
            // MUTANT (mut_00cfe020): steer down the other path.
            let take_first = if mutate_dispatch { flag != 3 } else { flag == 3 };
            if take_first {
                // Direct steering: out = base - 3*d1*s1 - d2*s2.
                let dx1 = sub(ax, rdf(b.wrapping_add(0x30)));
                let dy1 = sub(ay, rdf(b.wrapping_add(0x34)));
                let dz1 = sub(az, rdf(b.wrapping_add(0x38)));
                let dx2 = sub(ax, rdf(d.wrapping_add(0x30)));
                let dy2 = sub(ay, rdf(d.wrapping_add(0x34)));
                let dz2 = sub(az, rdf(d.wrapping_add(0x38)));
                let q1 = add(add(mul(dy1, dy1), mul(dx1, dx1)), mul(dz1, dz1));
                let s1 = lf_checker_rt::callee_cdecl!(SQRT_CALLEE, f64, q1.to_bits()) as f32;
                let q2 = add(add(mul(dy2, dy2), mul(dx2, dx2)), mul(dz2, dz2));
                let s2 = lf_checker_rt::callee_cdecl!(SQRT_CALLEE, f64, q2.to_bits()) as f32;
                let k3 = gf(G_K3);
                ox = sub(sub(rdf(d.wrapping_add(0x30)), mul(mul(dx1, s1), k3)), mul(dx2, s2));
                oy = sub(sub(rdf(d.wrapping_add(0x34)), mul(mul(dy1, s1), k3)), mul(dy2, s2));
                oz = sub(sub(rdf(d.wrapping_add(0x38)), mul(mul(dz1, s1), k3)), mul(dz2, s2));
            } else {
                // Clamped steering: out = avg - k*d1.
                let dx1 = sub(ax, rdf(b.wrapping_add(0x30)));
                let dy1 = sub(ay, rdf(b.wrapping_add(0x34)));
                let dz1 = sub(az, rdf(b.wrapping_add(0x38)));
                let q1 = add(add(mul(dy1, dy1), mul(dx1, dx1)), mul(dz1, dz1));
                let rb = sqrt(best);
                let rq = sqrt(q1);
                let mut t = mul(sub(rq, rb), gf(G_K005));
                t = if 0.0 > t { 0.0 } else { t };
                t = if t > 1.0 { 1.0 } else { t };
                let kk = div(add(mul(sub(1.0, t), gf(G_K20)), rb), rq);
                ox = add(mul(kk, sub(rdf(b.wrapping_add(0x30)), ax)), ax);
                oy = add(mul(kk, sub(rdf(b.wrapping_add(0x34)), ay)), ay);
                oz = add(mul(kk, sub(rdf(b.wrapping_add(0x38)), az)), az);
            }
        }

        // Tail: scale the delta to a unit vector times the constant.
        let dx = sub(ox, rdf(d.wrapping_add(0x30)));
        let dy = sub(oy, rdf(d.wrapping_add(0x34)));
        let dz = sub(oz, rdf(d.wrapping_add(0x38)));
        let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        // The original's ucomiss/lahf sequence normalises only a zero
        // distance to a zero scale; anything else divides, NaN included.
        let x1 = if d2 == 0.0 { 0.0 } else { div(1.0, sqrt(d2)) };
        let k5 = gf(G_K5);
        // Add orders are the original's per component.
        let fx = add(mul(mul(dx, x1), k5), rdf(d.wrapping_add(0x30)));
        let fy = add(rdf(d.wrapping_add(0x34)), mul(mul(dy, x1), k5));
        let fz = add(rdf(d.wrapping_add(0x38)), mul(mul(dz, x1), k5));
        wr32(out, fx.to_bits());
        wr32(out.wrapping_add(4), fy.to_bits());
        wr32(out.wrapping_add(8), fz.to_bits());
        wr32(out.wrapping_add(12), 0);
        1
    }
}

lf_checker_rt::export!(thiscall, rw_00cfe020(this: u32, arg0: u32, out: u32) -> u32 {
    unsafe { run_00cfe020(this, arg0, out, false) }
});

lf_checker_rt::export!(thiscall, mut_00cfe020(this: u32, arg0: u32, out: u32) -> u32 {
    unsafe { run_00cfe020(this, arg0, out, true) }
});
