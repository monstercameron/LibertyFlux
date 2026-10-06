// original: 0x008EDF20 weighted_proximity_mix (proposed)

/// Mix five proximity terms into one float answering how strongly a
/// query point is covered.
///
/// Seven stack words: four pointers to float triples (`v0` the query
/// point, `v1` a second point, `va`/`vb` a segment), an entity index, a
/// flag word and a float radius. A zero flag word, a missing entity, or a
/// missing entity core returns +0.0 at once. Otherwise five terms are
/// summed (in this order: segment term, alignment term, entity term,
/// height term, hub term) and the sum is returned on the x87 stack.
///
/// The segment term (flag bit 1): the squared lengths of `va` and `vb`
/// (negated when below zero, which a sum of squares never is) decide two
/// length flags against a tiny epsilon; the distance from `v0` to `vb` is
/// then mapped through a two-piece linear falloff (150.0/20.0, scaled by
/// 1.5) that is zero past 150. The alignment term (same flag, both length
/// flags set): the unit direction from `va` to `vb` dotted with the unit
/// direction from `vb` to `v0`, both normalized by the direction callee
/// (callee 2, in place through its frame pointer).
///
/// The entity term (flag bit 0) scans 32 cells: entity `i` is fetched
/// (callee 1, skipped when missing, coreless, or equal to the index
/// argument) and its position is either the entity's own or, when the
/// cell word falls in a live range (UNSIGNED compares against a limit
/// global), a position stored beside the cell. A cell whose position is
/// within 2500 of the far anchor, or (without an override) whose state
/// byte is set, is skipped. Any other cell within `radius` of `v0`
/// contributes through the near callee (callee 3): `((radius-dist)/radius)
/// * -4 - 1`; when the near callee declines, the far callee (callee 4)
/// may still subtract `((limit-dist)/limit) * 2` with a 60.0 limit when
/// flag bit 4 is set and 7.5 otherwise.
///
/// The table term always runs: up to `count` rows (SIGNED count) of a
/// global 32-byte-stride table are tested against `v0` (absolute height
/// window, then radial window) and each passing row subtracts its scaled
/// shortfall times 0.3. The height term (flag bit 3) maps the absolute
/// height gap between `v1` and `v0` through `(gap-5)*0.1` clamped to
/// [0, 1] (NaN survives both clamps) and scaled by -3. The hub term
/// always runs: the hub callee (callee 5) yields a hub, and when the
/// detail callee (callee 6) reports a point within 9 of `v0` the term is
/// -5 instead of 0.
///
/// Original: 0x008EDF20 (stdcall, seven stack words, f32 on x87; all
/// float arithmetic in the original's operand order).
lf_checker_rt::export!(stdcall, rw_008EDF20(
    v0: u32,
    v1: u32,
    va: u32,
    vb: u32,
    idx: u32,
    flags: u32,
    flt: u32,
) -> f32 {
    unsafe {
        const CALLEE_ENT: u32 = 1;
        const CALLEE_DIR: u32 = 2;
        const CALLEE_NEAR: u32 = 3;
        const CALLEE_FAR: u32 = 4;
        const CALLEE_HUB: u32 = 5;
        const CALLEE_DET: u32 = 6;
        const CELLS: u32 = 32;
        const CELL_BASE: u32 = 0x1177690;
        const CELL_STRIDE: u32 = 0x20;
        const CELL_END: u32 = 0x1177a90;
        const HUB_OBJ: u32 = 0x12e2420;

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
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        if flags == 0 {
            return 0.0;
        }
        let mut w20 = 0.0f32;
        let mut w24 = 0.0f32;
        let mut acc = 0.0f32;
        let mut w28 = 0.0f32;
        let mut w2c = 0.0f32;
        let p = lf_checker_rt::callee_cdecl!(CALLEE_ENT, u32, idx);
        if p == 0 {
            return 0.0;
        }
        let q = lf_checker_rt::callee_cdecl!(CALLEE_ENT, u32, idx);
        if rd32(q.wrapping_add(0x598)) == 0 {
            return 0.0;
        }
        let r = lf_checker_rt::callee_cdecl!(CALLEE_ENT, u32, idx);
        let core = rd32(r.wrapping_add(0x598));
        let radius = f32::from_bits(flt);
        // Scratch the detail callee takes: flag-2's difference vector when
        // that block ran (possibly normalized in place), zeros otherwise.
        // The call-time snapshot observes it either way.
        let mut dslot = [0.0f32; 3];

        if flags & 2 != 0 {
            let eps = *lf_checker_rt::global::<f32>(0xfe864c);
            let (va0, va1, va2) = (rdf(va), rdf(va.wrapping_add(4)), rdf(va.wrapping_add(8)));
            let mut la = add(add(mul(va0, va0), mul(va1, va1)), mul(va2, va2));
            if 0.0 > la {
                la = -la;
            }
            let dl = la > eps;
            let (vb0, vb1, vb2) = (rdf(vb), rdf(vb.wrapping_add(4)), rdf(vb.wrapping_add(8)));
            let mut lb = add(add(mul(vb0, vb0), mul(vb1, vb1)), mul(vb2, vb2));
            if 0.0 > lb {
                lb = -lb;
            }
            let cl = lb > eps;
            let (v00, v01, v02) = (rdf(v0), rdf(v0.wrapping_add(4)), rdf(v0.wrapping_add(8)));
            dslot = [sub(v00, vb0), sub(v01, vb1), sub(v02, vb2)];
            let d = dslot;
            let dist = add(add(mul(d[0], d[0]), mul(d[1], d[1])), mul(d[2], d[2])).sqrt();
            if cl && dl {
                let mut e = [sub(vb0, va0), sub(vb1, va1), sub(vb2, va2)];
                lf_checker_rt::callee_thiscall!(CALLEE_DIR, u32, e.as_mut_ptr() as u32);
                lf_checker_rt::callee_thiscall!(CALLEE_DIR, u32, dslot.as_mut_ptr() as u32);
                let d = dslot;
                let t1 = mul(e[0], d[0]);
                let t0 = mul(e[1], d[1]);
                let t1 = add(t1, t0);
                let t0 = mul(e[2], d[2]);
                w24 = add(t1, t0);
            }
            let hi = *lf_checker_rt::global::<f32>(0x10330d0);
            if hi > dist {
                let lo = *lf_checker_rt::global::<f32>(0x10330d4);
                let t = if lo > dist {
                    div(sub(lo, dist), lo)
                } else {
                    let one = *lf_checker_rt::global::<f32>(0xfe88e8);
                    sub(one, div(sub(dist, lo), sub(hi, lo)))
                };
                w20 = mul(t, *lf_checker_rt::global::<f32>(0xfe8960));
            }
        }

        if flags & 1 != 0 {
            let lim = *lf_checker_rt::global::<u32>(0x11735b4);
            let (c0, c1, c2) = (
                *lf_checker_rt::global::<f32>(0x10330e0),
                *lf_checker_rt::global::<f32>(0x10330e4),
                *lf_checker_rt::global::<f32>(0x10330e8),
            );
            let (v00, v01, v02) = (rdf(v0), rdf(v0.wrapping_add(4)), rdf(v0.wrapping_add(8)));
            let mut i = 0u32;
            let mut cell = CELL_BASE;
            while cell < CELL_END {
                let a = lf_checker_rt::callee_cdecl!(CALLEE_ENT, u32, i);
                if a != 0 {
                    let b = lf_checker_rt::callee_cdecl!(CALLEE_ENT, u32, i);
                    if rd32(b.wrapping_add(0x598)) != 0 && i != idx {
                        let c = lf_checker_rt::callee_cdecl!(CALLEE_ENT, u32, i);
                        let ec = rd32(c.wrapping_add(0x598));
                        let bp = rd32(ec.wrapping_add(0x20));
                        let (mut p3, mut p4, mut p5) =
                            (rdf(bp.wrapping_add(0x30)), rdf(bp.wrapping_add(0x34)), rdf(bp.wrapping_add(0x38)));
                        let mut dl = false;
                        let cw = *lf_checker_rt::global::<u32>(cell);
                        if cw != 0 && cw <= lim {
                            let cw2 = cw.wrapping_add(0xbb8);
                            if cw2 > lim {
                                p3 = *lf_checker_rt::global::<f32>(cell.wrapping_sub(0x10));
                                p4 = *lf_checker_rt::global::<f32>(cell.wrapping_sub(0xc));
                                p5 = *lf_checker_rt::global::<f32>(cell.wrapping_sub(8));
                                dl = true;
                            }
                        }
                        let d3 = sub(p3, c0);
                        let d4 = sub(p4, c1);
                        let d5 = sub(p5, c2);
                        let d2 = add(add(mul(d4, d4), mul(d3, d3)), mul(d5, d5));
                        if !(*lf_checker_rt::global::<f32>(0xfe8c74) > d2) {
                            if dl || ((ec.wrapping_add(0x211)) as *const u8).read() == 0 {
                                let dx = sub(v00, p3);
                                let dy = sub(v01, p4);
                                let dz = sub(v02, p5);
                                let s = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                                let dist = s.sqrt();
                                let this3 = rd32(core.wrapping_add(0x224));
                                let ok: u32 = lf_checker_rt::callee_thiscall!(
                                    CALLEE_NEAR, u32, this3, ec, 1
                                );
                                if ok as u8 != 0 {
                                    if radius > dist {
                                        let t = sub(radius, dist);
                                        let t = div(t, radius);
                                        let t = mul(t, *lf_checker_rt::global::<f32>(0xfe8dc8));
                                        let t = sub(t, *lf_checker_rt::global::<f32>(0xfe88e8));
                                        acc = add(acc, t);
                                    }
                                } else {
                                    let ok2: u32 = lf_checker_rt::callee_thiscall!(
                                        CALLEE_FAR, u32, this3, ec
                                    );
                                    if ok2 as u8 != 0 {
                                        let th = if flags & 0x10 != 0 {
                                            *lf_checker_rt::global::<f32>(0xfe8b80)
                                        } else {
                                            *lf_checker_rt::global::<f32>(0xfe8af4)
                                        };
                                        if th > dist {
                                            let t = div(sub(th, dist), th);
                                            let t = mul(t, *lf_checker_rt::global::<f32>(0xfe8a24));
                                            acc = sub(acc, t);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                i += 1;
                cell = cell.wrapping_add(CELL_STRIDE);
            }
        }

        let n = *lf_checker_rt::global::<i32>(0x1176e48);
        if n > 0 {
            let (v00, v01, v02) = (rdf(v0), rdf(v0.wrapping_add(4)), rdf(v0.wrapping_add(8)));
            let k6 = *lf_checker_rt::global::<f32>(0xfe87e8);
            let mut row = lf_checker_rt::relocated(0x1176e88);
            let mut count = n;
            while count != 0 {
                let f1 = sub(v02, rdf(row));
                let f2 = sub(v01, rdf(row.wrapping_sub(4)));
                let f3 = sub(v00, rdf(row.wrapping_sub(8)));
                // Absolute value; unordered (NaN) keeps f1, like comiss+jbe.
                let f4 = if f1 < 0.0 {
                    f32::from_bits(f1.to_bits() ^ 0x8000_0000)
                } else {
                    f1
                };
                let r0 = rdf(row.wrapping_add(0xc));
                if r0 > f4 {
                    let s = add(add(mul(f2, f2), mul(f3, f3)), mul(f1, f1));
                    let d = s.sqrt();
                    let r1 = rdf(row.wrapping_add(8));
                    if r1 > d {
                        let t = div(sub(r1, d), r1);
                        let t = mul(t, k6);
                        acc = sub(acc, t);
                    }
                }
                row = row.wrapping_add(0x20);
                count -= 1;
            }
        }

        if flags & 8 != 0 {
            let t0 = sub(rdf(v1.wrapping_add(8)), rdf(v0.wrapping_add(8)));
            // Absolute value: comiss+jbe keeps NaN and -0.0 as they compare.
            let mut t = if t0 < 0.0 {
                f32::from_bits(t0.to_bits() ^ 0x8000_0000)
            } else {
                t0
            };
            t = sub(t, *lf_checker_rt::global::<f32>(0xfe8ad8));
            t = mul(t, *lf_checker_rt::global::<f32>(0xfe879c));
            if t < 0.0 {
                t = 0.0;
            }
            if t > *lf_checker_rt::global::<f32>(0xfe88e8) {
                t = *lf_checker_rt::global::<f32>(0xfe88e8);
            }
            w28 = mul(t, *lf_checker_rt::global::<f32>(0xfe8dc0));
        }

        let hub: u32 =
            lf_checker_rt::callee_thiscall!(CALLEE_HUB, u32, lf_checker_rt::relocated(HUB_OBJ), v0, 0, 0, 0, 1);
        if hub != 0 {
            let r6: u32 =
                lf_checker_rt::callee_thiscall!(CALLEE_DET, u32, hub, dslot.as_mut_ptr() as u32);
            let dx = sub(rdf(r6), rdf(v0));
            let dy = sub(rdf(r6.wrapping_add(4)), rdf(v0.wrapping_add(4)));
            let dz = sub(rdf(r6.wrapping_add(8)), rdf(v0.wrapping_add(8)));
            let s = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            if *lf_checker_rt::global::<f32>(0xfe8b00) > s {
                w2c = *lf_checker_rt::global::<f32>(0xfe8dcc);
            }
        }

        add(add(add(add(w20, w24), acc), w28), w2c)
    }
});
