// original: 0x00CD2630 blend_pair_by_direction (proposed)

/// Blend two reference vectors by a direction weight and a gated factor.
///
/// `anchor` (P) and `target` (D) are vec3s whose difference gives a direction;
/// `vec_q` (Q) and `vec_r` (R) are the blended vec3s; `out_a`/`out_b` receive
/// one float each. `dist` (f) seeds the factor: the original also writes the
/// updated value back into its incoming stack slot, which a Rust rewrite has
/// no equivalent for, so the proof runs with the stack check off. `flag_b`
/// (low byte) zeroes a positive factor; `lo_lim` (g), `hi_lim` (h),
/// `neg_rate` (k1), `pos_rate` (k2) and `cap` (m) shape the factor curve;
/// `gate` (E) is a nullable object whose two one-byte checks (callees 2 and 3)
/// can clamp the factor to non-positive, non-negative or zero.
///
/// Direction: `d = D - P`, `d2 = dx*dx + dy*dy + dz*dz` in that order. The
/// inverse scale is `1/sqrt(d2)` unless `d2` is exactly zero (either sign),
/// in which case it is `+0`: `nx = inv*dx`, `ny = inv*dy`, `nz = inv*dz`.
///
/// Factor: `f' = f - sqrt(d2)`; `m1 = min(|f'|, h)` (NaN-aware: kept unless
/// strictly greater); if `g > m1` the factor is 0 and the curve call is
/// skipped. Otherwise `q = (m1-g)/(h-g)`, `s = k2` unless `f'` is negative
/// (then `k1`), `r = curve(q, s)` (callee 1, over xmm0/xmm1), and with the
/// sign selector `sg(f') = -1/0/+1` for negative/exact-zero/positive-or-NaN:
/// `t = -(sg(f')*r)`, then `t = sg(t)*m` when `m > |t|`. A positive `t` is
/// zeroed when `flag_b` is set.
///
/// Outputs: `*out_a = (Q.y*ny + Q.x*nx + Q.z*nz) * t` and
/// `*out_b = (R.y*ny + R.x*nx + R.z*nz) * t`, evaluated in that operand
/// order. Returns `out_b`.
///
/// Original: 0x00CD2630 (thiscall, fourteen stack words). Float operation
/// order is pinned with `black_box` throughout.
lf_checker_rt::export!(
    thiscall,
    rw_00CD2630(
        this: u32,
        out_a: u32,
        out_b: u32,
        anchor: u32,
        vec_q: u32,
        vec_r: u32,
        target: u32,
        dist: u32,
        flag_b: u32,
        lo_lim: u32,
        hi_lim: u32,
        neg_rate: u32,
        pos_rate: u32,
        cap: u32,
        gate: u32,
    ) -> u32 {
        unsafe {
            const SIGN: u32 = 0x8000_0000;
            const ABS_MASK: u32 = 0x7FFF_FFFF;
            const ONE: f32 = 1.0;
            const NEG_ONE: f32 = -1.0;
            const OBJ_STATE: u32 = 0x20;

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
            #[inline(always)]
            fn abs_bits(v: f32) -> f32 {
                f32::from_bits(v.to_bits() & ABS_MASK)
            }
            #[inline(always)]
            fn neg_bits(v: f32) -> f32 {
                f32::from_bits(v.to_bits() ^ SIGN)
            }
            /// The original's sign selector: -1.0 for negative, +1.0 for
            /// positive or NaN, 0.0 only for exact zeros (a-R02 fix).
            #[inline(always)]
            fn sign_sel(v: f32) -> f32 {
                if 0.0 > v {
                    NEG_ONE
                } else if v == 0.0 {
                    0.0
                } else {
                    ONE
                }
            }

            // Direction and its squared length.
            let dx = sub(rdf(target), rdf(anchor));
            let dy = sub(rdf(target + 4), rdf(anchor + 4));
            let dz = sub(rdf(target + 8), rdf(anchor + 8));
            let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            // Inverse scale unless the squared length is exactly zero: the
            // original's zero test jumps for greater, less, or unordered,
            // so only exact +0/-0 skip the divide (a-R02 fix: r-b431 read
            // it as NaN-only, which failed trials like 20 with -0 vs +0).
            let inv = if d2 == 0.0 {
                0.0
            } else {
                div(ONE, core::hint::black_box(d2).sqrt())
            };
            let nx = mul(inv, dx);
            let ny = mul(inv, dy);
            let nz = mul(inv, dz);

            // Factor curve.
            let f = f32::from_bits(dist);
            let g = f32::from_bits(lo_lim);
            let h = f32::from_bits(hi_lim);
            let fprime = sub(f, core::hint::black_box(d2).sqrt());
            // (The original writes fprime back to its incoming stack slot;
            // no Rust equivalent; the proof runs with checks.stack off.)
            let mut m1 = abs_bits(fprime);
            if m1 > h {
                m1 = h;
            }
            let mut t: f32;
            if g > m1 {
                t = 0.0;
            } else {
                let q = div(sub(m1, g), sub(h, g));
                let s = if !(0.0 > fprime) {
                    f32::from_bits(pos_rate)
                } else {
                    f32::from_bits(neg_rate)
                };
                // NOTE: an f32xmm0 stub answers in XMM0 *and* EAX; a Rust
                // cdecl f32 return reads ST0, so take the bits from EAX.
                let r: f32 = f32::from_bits(lf_checker_rt::callee_cdecl!(
                    1,
                    u32,
                    q.to_bits(),
                    s.to_bits()
                ));
                t = neg_bits(mul(sign_sel(fprime), r));
                let m = f32::from_bits(cap);
                if m > abs_bits(t) {
                    t = mul(sign_sel(t), m);
                }
            }
            if flag_b & 0xFF != 0 && t > 0.0 {
                t = 0.0;
            }

            // Gate checks: two one-byte callees over (gate, anchor/target).
            if gate != 0 && rd32(this + OBJ_STATE) != 0 {
                let al1 =
                    lf_checker_rt::callee_thiscall!(2, u32, gate, anchor) as u8;
                let al2 =
                    lf_checker_rt::callee_thiscall!(3, u32, gate, target) as u8;
                if al1 == 0 {
                    // (=0,=0): zero a positive t, then zero a negative t;
                    // (=0,!=0): only the second clamp.
                    if al2 == 0 && t > 0.0 {
                        t = 0.0;
                    }
                    if 0.0 > t {
                        t = 0.0;
                    }
                } else if al2 == 0 && t > 0.0 {
                    t = 0.0;
                }
            }

            // Outputs, in the original's operand order.
            let qx = rdf(vec_q);
            let qy = rdf(vec_q + 4);
            let qz = rdf(vec_q + 8);
            let oa = mul(add(add(mul(qy, ny), mul(qx, nx)), mul(qz, nz)), t);
            wrf(out_a, oa);
            let rx = rdf(vec_r);
            let ry = rdf(vec_r + 4);
            let rz = rdf(vec_r + 8);
            let ob = mul(add(add(mul(ry, ny), mul(rx, nx)), mul(rz, nz)), t);
            wrf(out_b, ob);
            out_b
        }
    }
);
