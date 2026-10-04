// original: 0x00d7cf30 input_stick_filter (proposed)

/// Filter a stick/angle input through rate limits and write four outputs.
///
/// The flag path (argument 4 set) copies `[a2+8]` into the limit; otherwise a
/// 500-tick rate gate either skips to the tail or runs the middle, which
/// refreshes the limit from a sensor word and a solver call.
///
/// `obj` is the filter object (vtable at `+0x0`, sub-object at `+0x20`,
/// gate counter at `+0x2c`, limit at `+0x1f00`, reference angle at `+0x1f74`).
/// `a1` is an angle, `a2` points at two floats and a dword, `a4`/`a5` are
/// flag bytes. The tail normalises angles to
/// [-pi, pi], scales by a global rate, clamps twice, and combines the
/// sub-object vectors with three vtable answers into `[obj+0x1eb4]`,
/// `[obj+0x1eb8]`, `[obj+0x1ebc]` and `[obj+0x1ec0]`. Returns `[obj+0x20]`.
///
/// Original: 0x00d7cf30 (cdecl, six stack words).
lf_checker_rt::export!(cdecl, rw_00d7cf30(obj: u32, a1: u32, a2: u32, _a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const VT_SLOT: u32 = 0xec;
        const GB4: u32 = 0x11735b4;
        const GB8: u32 = 0x11735b8;
        const GBC: u32 = 0x11735bc;
        const K_PI: u32 = 0xfe8aa0;
        const K_2PI: u32 = 0xfe8aec;
        const K_NPI: u32 = 0xfe8dc4;
        const K_TWO: u32 = 0xfe8a24;
        const K_RATE: u32 = 0x1056dd0; // 0.3
        const K_CAP: u32 = 0x1056dd4; // -3.0
        const K_UP: u32 = 0xfe879c; // 0.1
        const K_DN: u32 = 0xfe87d0; // 0.2
        const K_ONE: u32 = 0xfe88e8;
        const K_FIVE: u32 = 0xfe8ad8;
        const K_FAC: u32 = 0xfe876c; // 0.05
        const K_LO: u32 = 0xfe8d7c; // -0.5
        const K_M1: u32 = 0xfe8d94; // -1.0
        const K_M60: u32 = 0xfe8b80; // 60.0
        const K_M4A: u32 = 0x1056ddc; // 4.0
        const K_M4B: u32 = 0x1056dd8; // 4.0
        const K_M100: u32 = 0x1056de0; // 100.0
        const K_MSC: u32 = 0x1056de4; // -0.025
        const K_MSM: u32 = 0x1056de8; // -0.005
        const K_HI5: u32 = 0xfe888c; // 0.75
        const K_LO5: u32 = 0xe9b50c; // -0.75
        const K_HI7: u32 = 0xfe8874; // 0.7
        const K_LO7: u32 = 0xe9afac; // -0.7

        #[inline(always)]
        unsafe fn rd32(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(addr: u32) -> u16 {
            unsafe { (addr as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(addr: u32) -> f32 {
            unsafe { f32::from_bits(rd32(addr)) }
        }
        #[inline(always)]
        unsafe fn wrf(addr: u32, v: f32) {
            unsafe { (addr as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        unsafe fn gu(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        fn mul(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) * core::hint::black_box(y)
        }
        #[inline(always)]
        fn add(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) + core::hint::black_box(y)
        }
        #[inline(always)]
        fn sub(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) - core::hint::black_box(y)
        }
        #[inline(always)]
        fn div(x: f32, y: f32) -> f32 {
            core::hint::black_box(x) / core::hint::black_box(y)
        }
        #[inline(always)]
        fn wrap(mut ang: f32, pi: f32, pi2: f32, npi: f32) -> f32 {
            if ang > pi {
                loop {
                    ang = sub(ang, pi2);
                    if !(ang > pi) {
                        break;
                    }
                }
            }
            if npi > ang {
                loop {
                    ang = add(ang, pi2);
                    if !(npi > ang) {
                        break;
                    }
                }
            }
            ang
        }
        #[inline(always)]
        fn inv_len(len2: f32, one: f32) -> f32 {
            if len2 == 0.0 {
                0.0
            } else {
                core::hint::black_box(one) / core::hint::black_box(len2.sqrt())
            }
        }
        #[inline(always)]
        unsafe fn vcall(obj: u32, buf: u32) -> u32 {
            unsafe {
                let slot: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + VT_SLOT) as usize);
                slot(obj, buf)
            }
        }

        let pi = gf(K_PI);
        let pi2 = gf(K_2PI);
        let npi = gf(K_NPI);
        // Gates.
        if (a4 & 0xff) != 0 {
            wrf(obj + 0x1f00, rdf(a2 + 8));
        } else {
            let ctr = rd16(obj + 0x2c) as u32;
            let r0 = gu(GB8).wrapping_add(ctr) % 500;
            let r1 = gu(GB4).wrapping_add(ctr) % 500;
            if r1 < r0 {
                // Middle: refresh the limit from the sensor and a solver call.
                let mut mbuf = [0u32; 4];
                let mptr = mbuf.as_mut_ptr() as u32;
                let _z: u32 = lf_checker_rt::callee_thiscall!(6, u32, mptr);
                wrf(obj + 0x1f00, ((rd16(obj + 0xe68) as i16) as i32) as f32);
                let sub0 = rd32(obj + 0x20);
                let p1 = vcall(obj, mptr);
                let v0 = add(rdf(p1 + 4), rdf(sub0 + 0x34));
                let v1 = add(rdf(p1 + 8), rdf(sub0 + 0x38));
                let v2 = add(rdf(p1), rdf(sub0 + 0x30));
                // f32xmm0 answers arrive in eax as well (a Rust f32 return
                // would read ST0, which this stub leaves alone); take the bits.
                let r1 = f32::from_bits(lf_checker_rt::callee_cdecl!(3, u32, a1));
                let r2 = f32::from_bits(lf_checker_rt::callee_cdecl!(4, u32, a1));
                let inv = inv_len(add(add(mul(r1, r1), mul(r2, r2)), gf(K_ONE)), gf(K_ONE));
                let k60 = gf(K_M60);
                let mut x3 = mul(mul(r1, inv), k60);
                let mut x4 = mul(mul(r2, inv), k60);
                let mut x2 = mul(mul(inv, gf(K_M1)), k60);
                x3 = add(x3, v2);
                x4 = add(x4, v0);
                x2 = add(x2, v1);
                let buf_a = [v2.to_bits(), v0.to_bits(), v1.to_bits()];
                let buf_b = [x3.to_bits(), x4.to_bits(), x2.to_bits()];
                let mut buf_c = [0u32; 8];
                let r5: u32 = lf_checker_rt::callee_cdecl!(
                    5,
                    u32,
                    buf_a.as_ptr() as u32,
                    buf_b.as_ptr() as u32,
                    0,
                    buf_c.as_mut_ptr() as u32,
                    6,
                    1,
                    4
                );
                let mut e276m = 0.0f32;
                if r5 != 0 {
                    let t = f32::from_bits(buf_c[6]);
                    if t < 0.0 {
                        e276m = 0.0;
                    } else {
                        e276m = t;
                    }
                }
                let sum = add((((rd16(obj + 0xe6a) as i16) as i32) as f32), e276m);
                let mem = rdf(obj + 0x1f00);
                wrf(obj + 0x1f00, if mem > sum { mem } else { sum });
            }
        }
        // Tail.
        let subobj = rd32(obj + 0x20);
        let f1: f32 = lf_checker_rt::callee_cdecl!(2, f32, rdf(subobj + 0x10).to_bits(), rdf(subobj + 0x14).to_bits());
        let ang = wrap(sub(f1, rdf(obj + 0x1f74)), pi, pi2, npi);
        let rate = div(gf(K_RATE), gf(GBC));
        let e272 = add(mul(core::hint::black_box(rate), core::hint::black_box(ang)), f1);
        let mut tmp = [0u32; 4];
        let buf = tmp.as_mut_ptr() as u32;
        let p2 = vcall(obj, buf);
        let t = add(mul(rdf(p2 + 8), gf(K_TWO)), rdf(subobj + 0x38));
        let d = sub(rdf(obj + 0x1f00), t);
        let mut s = if d > 0.0 { mul(d, gf(K_UP)) } else { mul(d, gf(K_DN)) };
        if (a5 & 0xff) != 0 {
            let p3 = vcall(obj, buf);
            let cap = sub(gf(K_CAP), rdf(p3 + 8));
            if !(s > cap) {
                s = cap;
            }
        }
        let mut fac = gf(K_ONE);
        if s > gf(K_FIVE) {
            let v = sub(1.0, mul(sub(s, gf(K_FIVE)), gf(K_FAC)));
            fac = if v < 0.0 { 0.0 } else { v };
        }
        let mut e276 = s;
        if !(1.0 > s) {
            e276 = 1.0;
        } else if -0.5 > s {
            e276 = -0.5;
        }
        let norm2 = wrap(sub(f32::from_bits(a1), e272), pi, pi2, npi);
        let prod = mul(norm2, gf(K_LO));
        let c4 = if !(1.0 > prod) {
            1.0
        } else if !(-1.0 > prod) {
            prod
        } else {
            -1.0
        };
        wrf(obj + 0x1eb4, c4);
        let dx1 = sub(rdf(a2), rdf(subobj + 0x30));
        let dx2 = sub(rdf(a2 + 4), rdf(subobj + 0x34));
        let _p4 = vcall(obj, buf);
        let one = gf(K_ONE);
        let inv1 = inv_len(add(mul(rdf(subobj + 4), rdf(subobj + 4)), mul(rdf(subobj), rdf(subobj))), one);
        let e240 = mul(rdf(subobj), inv1);
        let e200 = mul(rdf(subobj + 4), inv1);
        let e272b = mul(inv1, 0.0);
        let inv2 = inv_len(add(mul(rdf(subobj + 0x14), rdf(subobj + 0x14)), mul(rdf(subobj + 0x10), rdf(subobj + 0x10))), one);
        let e176 = mul(rdf(subobj + 0x10), inv2);
        let e196 = mul(rdf(subobj + 0x14), inv2);
        let e192 = mul(inv2, 0.0);
        let p5 = vcall(obj, buf);
        let acc1 = add(
            add(mul(rdf(p5 + 4), e200), mul(e240, rdf(p5))),
            mul(rdf(p5 + 8), e272b),
        );
        let p6 = vcall(obj, buf);
        let p60 = rdf(p6);
        let p64 = rdf(p6 + 4);
        let p68 = rdf(p6 + 8);
        let x3 = add(add(mul(e196, dx2), mul(e176, dx1)), mul(e192, 0.0));
        let x1base = add(add(mul(p64, e196), mul(e176, p60)), mul(p68, e192));
        let mut x5 = mul(e200, dx2);
        x5 = add(x5, mul(e240, dx1));
        x5 = add(x5, mul(e272b, 0.0));
        x5 = sub(x5, mul(acc1, gf(K_M4B)));
        let x1 = mul(add(add(mul(rdf(subobj + 0x24), e200), mul(rdf(subobj + 0x20), e240)), mul(rdf(subobj + 0x28), e272b)), gf(K_M100));
        let x3 = sub(x3, mul(x1base, gf(K_M4A)));
        x5 = sub(x5, x1);
        x5 = mul(x5, gf(K_MSC));
        let c5 = if !(gf(K_HI5) > x5) {
            gf(K_HI5)
        } else if !(gf(K_LO5) > x5) {
            x5
        } else {
            gf(K_LO5)
        };
        let x3s = mul(x3, gf(K_MSM));
        wrf(obj + 0x1ebc, mul(c5, fac));
        let c3 = if !(gf(K_HI7) > x3s) {
            gf(K_HI7)
        } else if !(gf(K_LO7) > x3s) {
            x3s
        } else {
            gf(K_LO7)
        };
        wrf(obj + 0x1eb8, mul(c3, fac));
        wrf(obj + 0x1ec0, add(e276, gf(K_ONE)));
        subobj
    }
});
