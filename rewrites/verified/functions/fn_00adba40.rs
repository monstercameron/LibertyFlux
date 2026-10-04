// original: 0x00adba40 grid_noise_update (proposed)

/// Refresh the scrolling noise grid for the cell containing point `s`.
///
/// `s` points to `{ x: f32 @+0, y: f32 @+4, cap_in: f32 @+0x10,
/// num: u32 @+0x14, density: f32 @+0x18, w6: f32 @+0x1c, w20: f32 @+0x20,
/// w24: f32 @+0x24, den: u32 @+0x28 }`. The truncated `x - 100` and
/// `y - 100`, rounded to even, become the new grid origin (`G_X`, `G_Y`);
/// the old origin selects line-refresh calls (callees 1 and 2 step the
/// changed lines by 2, callee 3 handles a jump over 200, equal origins
/// call nothing). `cap = min(w value, CAP)` (NaN passes through) scales
/// everything after it.
///
/// Then: callee 4 (power) of `0.7 ^ cap`; a noise splatter loop whose
/// trip count is `density * 100 * 100` truncated, scattering
/// pseudo-random values from the multiply-carry generator at
/// `LCG0`/`LCG1` (multiplier `LCG_MUL`, mod-100 by magic multiply
/// `MOD100_MAGIC`) into field `A2`; a 100-step sine sweep (callee 5)
/// accumulating into the band table at `A3`; a 100x100 blend of field
/// `A1` (weights `W17`, `W08`) back into `A2`; per-item power scaling
/// for the `G_COUNT` items at `ITEMS` (each names three grid vertices
/// whose window must overlap the origin window); and a final
/// `cap`-weighted fold of `A2` into `A1`. Returns `0x9C40` always.
///
/// Original: 0x00ADBA40 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00adba40(s: u32) -> u32 {
    unsafe {
        const G_COUNT: u32 = 0x0155_2C60;
        const G_X: u32 = 0x0155_2C64;
        const G_Y: u32 = 0x0155_2C68;
        const G_RX: u32 = 0x0155_2C6C;
        const G_RY: u32 = 0x0155_2C70;
        const A1: u32 = 0x0155_2C78;
        const A2: u32 = 0x0155_C8B8;
        const A3: u32 = 0x0155_CA48;
        const ITEMS: u32 = 0x0155_2494;
        const GRID: u32 = 0x0158_E860;
        const CAP: u32 = 0x00E8_5F70;
        const M40: u32 = 0x0103_F504;
        const M05: u32 = 0x0103_F508;
        const P0: u32 = 0x0103_F50C;
        const LCG0: u32 = 0x0103_F4EC;
        const LCG1: u32 = 0x0103_F4F0;
        const LCG_MUL: u64 = 0x5CDC_FAA7;
        const MOD100_MAGIC: u64 = 0x51EB_851F;
        const C100: u32 = 0x00FE_8BB0;
        const C2M23: u32 = 0x00FE_864C;
        const C2: u32 = 0x00FE_8A24;
        const C1: u32 = 0x00FE_88E8;
        const C2PI: u32 = 0x00FE_8AEC;
        const W17: u32 = 0x00FE_87C8;
        const W08: u32 = 0x00FE_878C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16i(a: u32) -> i32 {
            unsafe { (a as *const u16).read_unaligned() as i16 as i32 }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() {
                return i32::MIN;
            }
            let t = x.trunc();
            if t >= 2147483648.0 || t < -2147483648.0 {
                i32::MIN
            } else {
                t as i32
            }
        }
        #[inline(always)]
        fn divmod100(x: i32) -> (i32, i32) {
            (x / 100, x % 100)
        }
        #[inline(always)]
        fn mod100(x: u32) -> u32 {
            let q = (((x as u64 * MOD100_MAGIC) >> 37) as u32).wrapping_mul(100);
            x.wrapping_sub(q)
        }

        let gx = rd32(lf_checker_rt::relocated(G_X)) as i32;
        let gy = rd32(lf_checker_rt::relocated(G_Y)) as i32;
        let tx = cvtt(rdf(s)).wrapping_sub(100);
        let nx = tx.wrapping_sub(tx >> 31) & !1;
        let ty = cvtt(rdf(s.wrapping_add(4))).wrapping_sub(100);
        let ny = ty.wrapping_sub(ty >> 31) & !1;
        wr32(lf_checker_rt::relocated(G_X), nx as u32);
        wr32(lf_checker_rt::relocated(G_Y), ny as u32);
        let rx = divmod100(nx / 2 + 100000).1;
        let ry = divmod100(ny / 2 + 100000).1;
        wr32(lf_checker_rt::relocated(G_RX), rx as u32);
        wr32(lf_checker_rt::relocated(G_RY), ry as u32);
        if !(gx == nx && gy == ny) {
            let dx = gx.wrapping_sub(nx).wrapping_abs();
            let dy = gy.wrapping_sub(ny).wrapping_abs();
            if dx > 200 || dy > 200 {
                let _: u32 = lf_checker_rt::callee_cdecl!(3, u32,);
            } else {
                let mut ox = gx;
                if ox < nx {
                    while ox < nx {
                        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, ox as u32);
                        ox = ox.wrapping_add(2);
                    }
                }
                if ox > nx {
                    loop {
                        ox = ox.wrapping_sub(2);
                        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32, ox as u32);
                        if ox <= nx {
                            break;
                        }
                    }
                }
                let mut oy = gy;
                if oy < ny {
                    while oy < ny {
                        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, oy as u32);
                        oy = oy.wrapping_add(2);
                    }
                }
                if oy > ny {
                    loop {
                        oy = oy.wrapping_sub(2);
                        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, oy as u32);
                        if oy <= ny {
                            break;
                        }
                    }
                }
            }
        }
        let capv = rdf(s.wrapping_add(0x10));
        let capc = rdf(lf_checker_rt::relocated(CAP));
        let cap = if capv > capc { capc } else { capv };
        let w28 = rdf(s.wrapping_add(0x1c));
        let w540 = mul(rdf(lf_checker_rt::relocated(M40)), cap);
        let w06 = mul(rdf(lf_checker_rt::relocated(M05)), cap);
        let pans = f32::from_bits(lf_checker_rt::callee_cdecl!(
            4,
            u32,
            rdf(lf_checker_rt::relocated(P0)).to_bits(),
            cap.to_bits()
        ));
        let a2 = lf_checker_rt::relocated(A2);
        let cnt = cvtt(mul(mul(rdf(s.wrapping_add(0x18)), rdf(lf_checker_rt::relocated(C100))), rdf(lf_checker_rt::relocated(C100))));
        if cnt > 0 {
            let c1 = rdf(lf_checker_rt::relocated(C2M23));
            let c2 = rdf(lf_checker_rt::relocated(C2));
            let c3 = rdf(lf_checker_rt::relocated(C1));
            let half = rdf(lf_checker_rt::relocated(M05));
            let mut l0 = rd32(lf_checker_rt::relocated(LCG0));
            let mut l1 = rd32(lf_checker_rt::relocated(LCG1));
            let mut n = cnt;
            loop {
                let t = (l0 as u64).wrapping_mul(LCG_MUL).wrapping_add(l1 as u64);
                l0 = t as u32;
                let s1 = (t >> 32) as u32;
                l1 = l0 & 0x7FFF_FFFF;
                let j = mod100(l1);
                let t = (l0 as u64).wrapping_mul(LCG_MUL).wrapping_add(s1 as u64);
                l0 = t as u32;
                l1 = (t >> 32) as u32;
                let mut s2 = mod100(l0 & 0x7FFF_FFFF);
                let t = (l0 as u64).wrapping_mul(LCG_MUL).wrapping_add(l1 as u64);
                l0 = t as u32;
                let a3 = (t >> 32) as u32;
                s2 = s2.wrapping_mul(100).wrapping_add(j);
                wr32(lf_checker_rt::relocated(LCG1), a3);
                l1 = a3;
                let mant = (l0 & 0x7F_FFFF) as f32;
                n -= 1;
                let mut v = mul(mant, c1);
                v = mul(v, c2);
                v = sub(v, c3);
                v = mul(v, w28);
                v = mul(v, half);
                v = mul(v, cap);
                wr32(lf_checker_rt::relocated(LCG0), l0);
                let p = a2.wrapping_add(s2.wrapping_mul(4));
                wrf(p, add(v, rdf(p)));
                if n == 0 {
                    break;
                }
            }
        }
        let num = rd32(s.wrapping_add(0x14));
        let den = rd32(s.wrapping_add(0x28));
        let rem = num % den;
        let v0 = mul(rdf(s.wrapping_add(0x24)), cap);
        let q = div(rdf(lf_checker_rt::relocated(C2PI)), (den as i32) as f32);
        let w = mul(rem as f32, q);
        let a3b = lf_checker_rt::relocated(A3);
        let s20 = rdf(s.wrapping_add(0x20));
        let mut xc = ny;
        let mut od = 0;
        let mut x1 = q;
        loop {
            let t = add(mul(xc as f32, s20), w);
            let s1: f32 = f32::from_bits(lf_checker_rt::callee_cdecl!(5, u32, t.to_bits(), x1.to_bits()));
            let x = mul(s1, v0);
            x1 = x;
            let si = divmod100(ry + od).1;
            let mut a = a3b.wrapping_add((si as u32).wrapping_mul(4));
            for _ in 0..10u32 {
                let p0 = a.wrapping_sub(0x190);
                wrf(p0, add(x, rdf(p0)));
                a = a.wrapping_add(0xFA0);
                for off in [0xFA0u32, 0xE10, 0xC80, 0xAF0, 0x960, 0x7D0, 0x640, 0x4B0, 0x320] {
                    let p = a.wrapping_sub(off);
                    wrf(p, add(x, rdf(p)));
                }
            }
            xc += 2;
            od += 1;
            if od >= 100 {
                break;
            }
        }
        let a1 = lf_checker_rt::relocated(A1);
        let w17 = rdf(lf_checker_rt::relocated(W17));
        let w08 = rdf(lf_checker_rt::relocated(W08));
        let mut o = 0;
        loop {
            let s1 = divmod100(rx + o).1;
            let r1 = divmod100(s1 + 1).1;
            let r2 = divmod100(s1 + 99).1;
            let aa = if o == 0 { s1 } else { r2 };
            let cc = if o == 99 { s1 } else { r1 };
            let (ba, bc, bs) = (aa * 100, cc * 100, s1 * 100);
            let mut i = 0;
            loop {
                let u = divmod100(ry + i).1;
                let r1 = divmod100(u + 1).1;
                let c2 = divmod100(u + 99).1;
                let tt = if i == 0 { u } else { c2 };
                let s2 = if i == 99 { u } else { r1 };
                let mut v1 = rdf(a1.wrapping_add(((ba + u) as u32).wrapping_mul(4)));
                v1 = add(v1, rdf(a1.wrapping_add(((bc + u) as u32).wrapping_mul(4))));
                v1 = add(v1, rdf(a1.wrapping_add(((bs + tt) as u32).wrapping_mul(4))));
                v1 = add(v1, rdf(a1.wrapping_add(((bs + s2) as u32).wrapping_mul(4))));
                v1 = mul(v1, w17);
                let mut v0 = rdf(a1.wrapping_add(((bc + tt) as u32).wrapping_mul(4)));
                v0 = add(v0, rdf(a1.wrapping_add(((bc + s2) as u32).wrapping_mul(4))));
                v0 = add(v0, rdf(a1.wrapping_add(((ba + tt) as u32).wrapping_mul(4))));
                v0 = add(v0, rdf(a1.wrapping_add(((ba + s2) as u32).wrapping_mul(4))));
                v0 = mul(v0, w08);
                v1 = add(v1, v0);
                let base = rdf(a1.wrapping_add(((bs + u) as u32).wrapping_mul(4)));
                let t = mul(base, w06);
                let d = sub(base, v1);
                let kp = a2.wrapping_add(((bs + u) as u32).wrapping_mul(4));
                wrf(kp, mul(sub(sub(rdf(kp), mul(d, w540)), t), pans));
                i += 1;
                if i >= 100 {
                    break;
                }
            }
            o += 1;
            if o >= 100 {
                break;
            }
        }
        let mut n = rd32(lf_checker_rt::relocated(G_COUNT)) as i32;
        if n > 0 {
            let cx = rd32(lf_checker_rt::relocated(G_X)) as i32;
            let cy = rd32(lf_checker_rt::relocated(G_Y)) as i32;
            let grid = lf_checker_rt::relocated(GRID);
            let mut dp = lf_checker_rt::relocated(ITEMS);
            loop {
                let gx0 = rd16i(grid.wrapping_add((rd16i(dp).wrapping_mul(8)) as u32));
                let mut run = gx0 < cx.wrapping_add(200);
                let mut gx1 = 0;
                let mut gy0 = 0;
                let mut gy1 = 0;
                if run {
                    gx1 = rd16i(grid.wrapping_add((rd16i(dp.wrapping_add(2)).wrapping_mul(8)) as u32));
                    run = gx1 >= cx;
                }
                if run {
                    gy0 = rd16i(grid.wrapping_add((rd16i(dp).wrapping_mul(8)) as u32).wrapping_add(2));
                    run = gy0 < cy.wrapping_add(200);
                }
                if run {
                    gy1 = rd16i(grid.wrapping_add((rd16i(dp.wrapping_add(4)).wrapping_mul(8)) as u32).wrapping_add(2));
                    run = gy1 >= cy;
                }
                if run {
                    let p = f32::from_bits(lf_checker_rt::callee_cdecl!(
                        4,
                        u32,
                        rd32(dp.wrapping_sub(4)),
                        cap.to_bits()
                    ));
                    let x0 = gx0.max(cx);
                    let x1 = gx1.min(cx.wrapping_add(200));
                    let y0 = gy0.max(cy);
                    let y1 = gy1.min(cy.wrapping_add(200));
                    // Note: cdq/sub/sar halves (x - sign(x)); the cdq
                    // clobbers the origin, so there is no -cx/-cy here.
                    let hx = (x0.wrapping_sub(x0 >> 31) >> 1).wrapping_add(100000);
                    let qx = divmod100(hx).1;
                    let hy = (y0.wrapping_sub(y0 >> 31) >> 1).wrapping_add(100000);
                    let qy = divmod100(hy).1;
                    if x0 <= x1 {
                        let mut nnx = ((x1 - x0) >> 1) + 1;
                        let mut xcur = qx;
                        while nnx > 0 {
                            if y0 <= y1 {
                                let mut nny = ((y1 - y0) >> 1) + 1;
                                let mut ycur = qy;
                                while nny > 0 {
                                    let idx = (xcur * 100 + ycur) as u32;
                                    let pp = a2.wrapping_add(idx.wrapping_mul(4));
                                    wrf(pp, mul(p, rdf(pp)));
                                    ycur = divmod100(ycur + 1).1;
                                    nny -= 1;
                                }
                            }
                            xcur = divmod100(xcur + 1).1;
                            nnx -= 1;
                        }
                    }
                }
                dp = dp.wrapping_add(16);
                n -= 1;
                if n == 0 {
                    break;
                }
            }
        }
        for k in 0..2500u32 {
            let b = k.wrapping_mul(16);
            for j in 0..4u32 {
                // Note: the original bumps the cursor between the A2 read
                // and the A1 read/write, so A1 runs 16 bytes ahead of A2.
                let p1 = a1.wrapping_add(b).wrapping_add(j.wrapping_mul(4));
                let p2 = a2.wrapping_add(b).wrapping_add(j.wrapping_mul(4));
                wrf(p1, add(mul(rdf(p2), cap), rdf(p1)));
            }
        }
        0x9C40
    }
});
