// original: 0x00c9c990 ped_task_aim_update (proposed)

/// Drive two aimed angles of a ped task toward computed targets, then clamp them.
///
/// `this` is the task object (status bits at `+0xC8`); `a1` is its parameter
/// block (category index as a signed 16-bit word at `+0x2E`, fallback table
/// pointer at `+0x100`, vtable with probe slots at `+0xA0`/`+0xE0`); `a3`
/// points at three floats (aim direction); `a4` is a blend weight passed to the
/// finalize helper; `a5` is a flag word; `a6`/`a7` are approach rates; `a8`/`a9`
/// point at the two driven angles. `a2` is never read.
///
/// Algorithm: resolve two helper objects through the global pointer table at
/// their category index and query each (slot `+0x38`, codes 7 and 6); the two
/// answers index rows of a 0xE0-byte-stride table later. Project the aim
/// direction through a 3x3 basis into three components; when the second and
/// third are both below 0.001 in magnitude the second snaps to 0.001 and the
/// third to zero. Fold the first two through the double-precision angle helper
/// (twice) into a base angle clamped to [-1.57, 1.57], scaled, and mixed with a
/// wrapped-angle helper. Move each driven angle toward its target by at most
/// its rate (a global rate multiplier, zero in a pristine image, makes this a
/// no-op step that still records status bits 1 and 8 on a snap). Run two
/// basis-evaluation helpers into one 9-word buffer, combine them through the
/// constant matrix into two orthonormal-ish frames (each normalized by
/// reciprocal square root, skipped when the squared length is exactly zero),
/// and derive two more angles through the double helper and the single-float
/// arc helper. Feed three triples to the pose solver, run the finalize helper,
/// then clamp the angles into their [low, high] windows from the category
/// row (or flag overrides): [a9] against the first window, [a8] against the
/// second (crossed relative to the approach step), setting status bits
/// 0x10/0x04 and clearing 0x08/0x01. Returns the finalize helper's status word.
///
/// Float comparisons follow `comiss` exactly: every ordered comparison is
/// written so NaN takes the fall-through path the original's jump takes
/// (`!(a > b)` for jump-if-below-or-equal, plain `>` for jump-if-above). The
/// three `ucomiss`-plus-`lahf` tests are exact-equality tests: normalize when
/// the value is not exactly zero (NaN normalizes), take the arc path unless
/// both inputs are exactly zero. Integer flag tests are exact bit tests. The
/// float operation order is the original's throughout.
///
/// Original: 0x00c9c990 (thiscall, nine stack words).
lf_checker_rt::export!(thiscall, rw_00c9c990(this: u32, a1: u32, _a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, a9: u32) -> u32 {
    unsafe {
        const GTABLE: u32 = 0x01295CD8;
        const VT_SLOT_QUERY: u32 = 0x38;
        const VT_SLOT_PROBE: u32 = 0xA0;
        const VT_SLOT_TABLE: u32 = 0xE0;
        const PARAM_INDEX: u32 = 0x2E;
        const PARAM_FALLBACK: u32 = 0x100;
        const THIS_STATUS: u32 = 0xC8;
        const ROW_STRIDE: u32 = 0xE0;
        const RATE_GLOBAL: u32 = 0x011735BC;
        const DEG2RAD: u32 = 0x00FE8728;
        const SNAP_SMALL: u32 = 0x00FE86B4;
        const CLAMP_HI: u32 = 0x00E9B4EC;
        const CLAMP_LO: u32 = 0x00E9B510;
        const UNSCALE: u32 = 0x00E9B4E0;
        const WRAP_BIAS_BITS: u32 = 0xBECCCCCD;
        const DATA_A: u32 = 0x0110DB00;
        const DATA_B: u32 = 0x0110DB70;
        const MAT: u32 = 0x01110090;
        const ONE_F: u32 = 0x00FE88E8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { (a as *const f32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut f32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn grdf(va: u32) -> f32 {
            unsafe { (lf_checker_rt::relocated(va) as *const f32).read_unaligned() }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsqrt(a: f32) -> f32 {
            core::hint::black_box(a).sqrt()
        }
        #[inline(always)]
        fn fabs(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & 0x7FFF_FFFF)
        }
        #[inline(always)]
        fn fneg(a: f32) -> f32 {
            f32::from_bits(a.to_bits() ^ 0x8000_0000)
        }
        #[inline(always)]
        unsafe fn angle2(x: f64, y: f64) -> f64 {
            // Double-precision two-argument angle helper (callee 3): the two
            // doubles travel as four stack words, low then high each; the
            // answer arrives in edx:eax.
            let bx = x.to_bits();
            let by = y.to_bits();
            let ans: u64 = lf_checker_rt::callee_cdecl!(
                3, u64,
                bx as u32, (bx >> 32) as u32,
                by as u32, (by >> 32) as u32
            );
            f64::from_bits(ans)
        }

        // Block 1: resolve the category helpers and query them.
        let index = ((a1.wrapping_add(PARAM_INDEX)) as *const i16).read_unaligned() as i32 as u32;
        let tobj = rd32(lf_checker_rt::relocated(GTABLE).wrapping_add(index.wrapping_mul(4)));
        let query: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(tobj).wrapping_add(VT_SLOT_QUERY)) as usize);
        let r1 = query(tobj, 7);
        let p2: u32 = lf_checker_rt::callee_thiscall!(2, u32, a1, r1);
        let r3 = query(tobj, 6);
        let p4: u32 = lf_checker_rt::callee_thiscall!(2, u32, a1, r3);

        // Block 2: project the aim direction through the basis.
        let dx = fsub(rdf(a3), rdf(p2.wrapping_add(0x30)));
        let mut dy = fsub(rdf(a3.wrapping_add(4)), rdf(p2.wrapping_add(0x34)));
        let mut dz = fsub(rdf(a3.wrapping_add(8)), rdf(p2.wrapping_add(0x38)));
        let mut my = rdf(p4.wrapping_add(4));
        let mut px = rdf(p4);
        let mut mz = rdf(p4.wrapping_add(8));
        let mut m10 = rdf(p4.wrapping_add(0x10));
        let mut m18 = rdf(p4.wrapping_add(0x18));
        my = fmul(my, dy);
        px = fmul(px, dx);
        mz = fmul(mz, dz);
        my = fadd(my, px);
        m10 = fmul(m10, dx);
        m18 = fmul(m18, dz);
        dz = fmul(dz, rdf(p4.wrapping_add(0x28)));
        let t48 = my;
        my = rdf(p4.wrapping_add(0x20));
        let t48 = fadd(t48, mz);
        mz = rdf(p4.wrapping_add(0x14));
        mz = fmul(mz, dy);
        dy = fmul(dy, rdf(p4.wrapping_add(0x24)));
        mz = fadd(mz, m10);
        my = fmul(my, dx);
        let mut t28 = fadd(mz, m18);
        dy = fadd(dy, my);
        let mut t38 = fadd(dy, dz);
        let snap = grdf(SNAP_SMALL);
        if snap > fabs(t28) && snap > fabs(t38) {
            t28 = snap;
            t38 = 0.0;
        }

        // Block 3: base angle through the double helper, wrap and mix.
        let d1 = angle2(f64::from(t38), f64::from(t28));
        let mut q1 = fmul(t38, t38);
        let q4 = fmul(t28, t28);
        let mut t08 = d1 as f32;
        let e0 = f64::from(fneg(t48));
        q1 = fadd(q1, q4);
        q1 = fsqrt(q1);
        let d2 = angle2(e0, f64::from(q1));
        let t18 = d2 as f32;
        (this.wrapping_add(THIS_STATUS) as *mut u32).write_unaligned(0);
        let mut x0 = t08;
        let hi = grdf(CLAMP_HI);
        if !(hi > x0) {
            x0 = hi;
        } else {
            let lo = grdf(CLAMP_LO);
            if !(x0 > lo) {
                x0 = lo;
            }
        }
        t08 = x0;
        x0 = fmul(x0, grdf(UNSCALE));
        x0 = fabs(x0);
        let r7: f32 = lf_checker_rt::callee_cdecl!(4, f32, t18.to_bits(), WRAP_BIAS_BITS, x0.to_bits());
        let mut m184 = t18;
        if m184 > r7 {
            m184 = r7;
        }

        // Block 4: step each driven angle toward its target.
        let rate = grdf(RATE_GLOBAL);
        let deg = grdf(DEG2RAD);
        let a8v = rdf(a8);
        let thr3 = fmul(fmul(f32::from_bits(a6), deg), rate);
        if thr3 > fabs(fsub(a8v, t08)) {
            wrf(a8, t08);
            let st = rd32(this.wrapping_add(THIS_STATUS));
            (this.wrapping_add(THIS_STATUS) as *mut u32).write_unaligned(st | 1);
        } else if t08 > a8v {
            wrf(a8, fadd(a8v, thr3));
        } else if a8v > t08 {
            wrf(a8, fsub(a8v, thr3));
        }
        let a9v = rdf(a9);
        let thr2 = fmul(fmul(f32::from_bits(a7), deg), rate);
        if thr2 > fabs(fsub(a9v, m184)) {
            wrf(a9, m184);
            let st = rd32(this.wrapping_add(THIS_STATUS));
            (this.wrapping_add(THIS_STATUS) as *mut u32).write_unaligned(st | 8);
        } else if m184 > a9v {
            wrf(a9, fadd(a9v, thr2));
        } else if a9v > m184 {
            wrf(a9, fsub(a9v, thr2));
        }

        // Block 5: category state query, then the first basis evaluation.
        let r8: u32 = lf_checker_rt::callee_thiscall!(5, u32, a1, r1);
        // The evaluation helper fills 11 words; the function reads all but
        // words 3 and 7.
        let mut w = [0u32; 11];
        lf_checker_rt::callee_thiscall!(6, u32, w.as_mut_ptr() as u32, lf_checker_rt::relocated(DATA_A), rdf(a8).to_bits());
        let w0 = f32::from_bits(w[0]);
        let w1 = f32::from_bits(w[1]);
        let w2 = f32::from_bits(w[2]);
        let w4 = f32::from_bits(w[4]);
        let w5 = f32::from_bits(w[5]);
        let w6 = f32::from_bits(w[6]);
        let w7 = f32::from_bits(w[8]);
        let w8 = f32::from_bits(w[9]);
        let w9 = f32::from_bits(w[10]);

        // Block 6: combine the first evaluation through the constant matrix.
        let m90 = grdf(MAT.wrapping_add(0x00));
        let m94 = grdf(MAT.wrapping_add(0x04));
        let mx98 = grdf(MAT.wrapping_add(0x08));
        let mxa0 = grdf(MAT.wrapping_add(0x10));
        let ma4 = grdf(MAT.wrapping_add(0x14));
        let ma8 = grdf(MAT.wrapping_add(0x18));
        let mb0 = grdf(MAT.wrapping_add(0x20));
        let mb4 = grdf(MAT.wrapping_add(0x24));
        let mb8 = grdf(MAT.wrapping_add(0x28));
        let mut c3 = fmul(w1, ma4);
        let mut c0 = fmul(w0, m94);
        c3 = fadd(c3, c0);
        c0 = fmul(w2, mb4);
        let mut c4 = fmul(w8, mxa0);
        c3 = fadd(c3, c0);
        c0 = fmul(mx98, w0);
        let t18 = c3;
        c3 = fmul(ma8, w1);
        c3 = fadd(c3, c0);
        c0 = fmul(mb8, w2);
        c3 = fadd(c3, c0);
        c0 = fmul(w4, m90);
        let t38 = c3;
        c3 = w5;
        let mut c7 = fmul(w5, mxa0);
        let mut c6 = fmul(w5, ma4);
        c3 = fmul(w5, ma8);
        c7 = fadd(c7, c0);
        c0 = fmul(w6, mb0);
        c7 = fadd(c7, c0);
        c0 = fmul(w4, m94);
        let mut c2 = fmul(w4, mx98);
        c6 = fadd(c6, c0);
        c0 = fmul(w6, mb4);
        let mut c1 = fmul(w6, mb8);
        c6 = fadd(c6, c0);
        c3 = fadd(c3, c2);
        c2 = w7;
        c0 = fmul(w7, m90);
        c3 = fadd(c3, c1);
        c1 = f32::from_bits(w[10]);
        c4 = fadd(c4, c0);
        c0 = fmul(w9, mb0);
        let t28 = c3;
        c3 = fmul(w8, ma4);
        c4 = fadd(c4, c0);
        c0 = fmul(w7, m94);
        c3 = fadd(c3, c0);
        c0 = fmul(w9, mb4);
        let mut c5 = fmul(w8, ma8);
        c2 = fmul(w7, mx98);
        c1 = fmul(w9, mb8);
        c3 = fadd(c3, c0);
        c0 = fmul(m90, w0);
        c5 = fadd(c5, c2);
        let m88 = c7;
        let m8c = c6;
        c5 = fadd(c5, c1);
        c1 = fmul(mxa0, w1);
        let m98 = c4;
        let m9c = c3;
        c1 = fadd(c1, c0);
        c0 = fmul(mb0, w2);
        let ma0 = c5;
        c1 = fadd(c1, c0);
        let m7c = t18;
        let m80 = t38;
        let m90v = t28;
        let m78 = c1;

        // Block 7: second evaluation into the same buffer, second combination.
        lf_checker_rt::callee_thiscall!(6, u32, w.as_mut_ptr() as u32, lf_checker_rt::relocated(DATA_B), rdf(a9).to_bits());
        let v0 = f32::from_bits(w[0]);
        let v1 = f32::from_bits(w[1]);
        let v2 = f32::from_bits(w[2]);
        let v4 = f32::from_bits(w[4]);
        let v5 = f32::from_bits(w[5]);
        let v6 = f32::from_bits(w[6]);
        let v7 = f32::from_bits(w[8]);
        let v8 = f32::from_bits(w[9]);
        let v9 = f32::from_bits(w[10]);
        c1 = fmul(v1, m8c);
        c0 = fmul(v0, m7c);
        c2 = v4;
        c3 = v5;
        c1 = fadd(c1, c0);
        c0 = fmul(v2, m9c);
        c3 = fmul(c3, m88);
        c1 = fadd(c1, c0);
        c0 = fmul(m80, v0);
        c7 = m78;
        c6 = m98;
        let t18b = c1;
        c1 = fmul(m90v, v1);
        c1 = fadd(c1, c0);
        c0 = fmul(ma0, v2);
        c1 = fadd(c1, c0);
        c0 = fmul(v4, c7);
        let t38b = c1;
        c1 = v6;
        c3 = fadd(c3, c0);
        c0 = fmul(v6, c6);
        c3 = fadd(c3, c0);
        c0 = v5;
        let t28b = c3;
        c3 = fmul(v5, m8c);
        c0 = fmul(v4, m7c);
        c2 = fmul(v4, m80);
        c3 = fadd(c3, c0);
        c0 = fmul(v6, m9c);
        c1 = fmul(v6, ma0);
        c3 = fadd(c3, c0);
        c0 = fmul(v5, m90v);
        let t48b = c3;
        c0 = fadd(c0, c2);
        c0 = fadd(c0, c1);
        let t08b = c0;
        c2 = v7;
        c5 = v8;
        c1 = v9;
        c4 = fmul(v8, m88);
        c0 = fmul(v7, c7);
        c7 = fmul(c7, v0);
        c4 = fadd(c4, c0);
        c0 = fmul(v9, c6);
        c6 = fmul(c6, v2);
        c4 = fadd(c4, c0);
        c3 = fmul(v8, m8c);
        c0 = fmul(v7, m7c);
        c5 = fmul(v8, m90v);
        c3 = fadd(c3, c0);
        c2 = fmul(v7, m80);
        c0 = fmul(v9, m9c);
        c1 = fmul(v9, ma0);
        c3 = fadd(c3, c0);
        c0 = fmul(m88, v1);
        c5 = fadd(c5, c2);
        c0 = fadd(c0, c7);
        let m98b = c4;
        let m9cb = c3;
        c5 = fadd(c5, c1);
        c0 = fadd(c0, c6);
        let ma0b = c5;
        let m78b = c0;
        let m7cb = t18b;
        let m80b = t38b;
        let m88b = t28b;
        let m8cb = t48b;
        let m90b = t08b;

        // Block 8: probe the parameter block's table (first site).
        // (Four loads from [r8+0x30..0x3C] feed only dead stores; not replicated.)
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(a1).wrapping_add(VT_SLOT_PROBE)) as usize);
        let tabobj: u32 = if probe(a1) == 0 {
            rd32(a1.wrapping_add(PARAM_FALLBACK))
        } else {
            let inner = probe(a1);
            let fetch: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                rd32(rd32(inner).wrapping_add(VT_SLOT_TABLE)) as usize,
            );
            fetch(inner)
        };
        let row = r1
            .wrapping_mul(ROW_STRIDE)
            .wrapping_add(rd32(rd32(tabobj.wrapping_add(4))));

        // Block 9: category row windows, with flag overrides.
        let mut b38 = rdf(row.wrapping_add(0xB0));
        let mut b28 = rdf(row.wrapping_add(0xC0));
        let mut b60 = rdf(row.wrapping_add(0xB8));
        let mut b48 = rdf(row.wrapping_add(0xC8));
        let bc = rdf(row.wrapping_add(0xB4));
        let dc = rdf(row.wrapping_add(0xC4));
        if ((a5 as u8) as i8) < 0 {
            b60 = grdf(0x00ED6F00);
            b48 = grdf(0x00ED6EEC);
        }
        if a5 & 0x200 != 0 {
            b60 = grdf(0x00ED6F04);
            b48 = grdf(0x00ED6EF0);
        }
        if a5 & 0x40 != 0 {
            b38 = grdf(0x00ED6F08);
            b28 = grdf(0x00E9B4E4);
        }
        if a5 & 0x100 != 0 {
            b38 = grdf(0x00FE8DA0);
            b28 = grdf(0x00FE8978);
        }

        // Block 10: normalize the combined frame, cross products.
        let mut n6 = m9cb;
        let mut n7 = m98b;
        let mut n4 = ma0b;
        let mut q0 = fmul(n7, n7);
        let mut q1 = fmul(n6, n6);
        q1 = fadd(q1, q0);
        q0 = fmul(n4, n4);
        q1 = fadd(q1, q0);
        let one = grdf(ONE_F);
        let rcp1 = if q1 != 0.0 { fdiv(one, fsqrt(q1)) } else { 0.0 };
        let mut s5 = m90b;
        n6 = fmul(n6, rcp1);
        n4 = fmul(n4, rcp1);
        n7 = fmul(n7, rcp1);
        let mut s1 = m8cb;
        q0 = fmul(s1, n4);
        let mut e18 = n6;
        s1 = fmul(s1, n7);
        let mut e8 = q0;
        q0 = fmul(s5, n6);
        let mut g6 = e8;
        s5 = fmul(s5, n7);
        g6 = fsub(g6, q0);
        let d0v = n4;
        e8 = g6;
        g6 = m88b;
        q0 = fmul(g6, n4);
        n4 = e18;
        g6 = fmul(g6, n4);
        s5 = fsub(s5, q0);
        q0 = e8;
        g6 = fsub(g6, s1);
        q0 = fmul(q0, q0);
        s1 = s5;
        s1 = fmul(s1, s5);
        s1 = fadd(s1, q0);
        q0 = g6;
        q0 = fmul(q0, g6);
        s1 = fadd(s1, q0);
        let rcp3 = if s1 != 0.0 { fdiv(one, fsqrt(s1)) } else { 0.0 };
        q0 = fmul(rcp3, e8);
        e18 = q0;
        q0 = fmul(rcp3, s5);
        s5 = e18;
        let mut i1 = q0;
        e8 = q0;
        q0 = fmul(s5, n4);
        i1 = fmul(i1, n7);
        let g3 = fmul(rcp3, g6);
        i1 = fsub(i1, q0);
        q0 = e8;
        let m70 = g3;
        e18 = i1;
        e8 = if s5 != 0.0 || q0 != 0.0 {
            angle2(f64::from(q0), f64::from(s5)) as f32
        } else {
            0.0
        };

        // Block 11: single-float arc plus the second double angle.
        let s_val =
            f32::from_bits(lf_checker_rt::callee_cdecl!(9, u32, fneg(m70).to_bits()));
        let n2 = d0v;
        let n3 = e18;
        let m70b = s_val;
        let atanb = if n2 == 0.0 && n3 == 0.0 {
            0.0
        } else {
            angle2(f64::from(n3), f64::from(n2)) as f32
        };

        // Block 12: pose solver and finalize helper.
        let mut cb1 = [b28.to_bits(), dc.to_bits(), b48.to_bits()];
        let mut cb2 = [b38.to_bits(), bc.to_bits(), b60.to_bits()];
        let mut cb3 = [atanb.to_bits(), m70b.to_bits(), e8.to_bits()];
        lf_checker_rt::callee_cdecl!(
            10, u32,
            cb3.as_mut_ptr() as u32,
            cb2.as_mut_ptr() as u32,
            cb1.as_mut_ptr() as u32
        );
        let mut cbuf = [
            m78b.to_bits(),
            m7cb.to_bits(),
            m80b.to_bits(),
            0,
            m88b.to_bits(),
            m8cb.to_bits(),
            m90b.to_bits(),
            0,
        ];
        lf_checker_rt::callee_thiscall!(11, u32, cbuf.as_mut_ptr() as u32, cb3.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(12, u32, r8, r8, cbuf.as_mut_ptr() as u32, a4);

        // Block 13: clamp the angles into their windows. Note the crossing:
        // the first clamp drives [a9] against the b48/b60 window, the second
        // drives [a8] against b28/b38 (the approach step paired them the
        // other way round).
        let a9c = rdf(a9);
        if a9c > b48 {
            wrf(a9, b48);
            let st = rd32(this.wrapping_add(THIS_STATUS));
            (this.wrapping_add(THIS_STATUS) as *mut u32)
                .write_unaligned((st & !8) | 0x10);
        } else if b60 > a9c {
            wrf(a9, b60);
            let st = rd32(this.wrapping_add(THIS_STATUS));
            (this.wrapping_add(THIS_STATUS) as *mut u32)
                .write_unaligned((st & !8) | 0x10);
        }
        let a8c = rdf(a8);
        if a8c > b28 {
            wrf(a8, b28);
            let st = rd32(this.wrapping_add(THIS_STATUS));
            (this.wrapping_add(THIS_STATUS) as *mut u32)
                .write_unaligned((st & !1) | 4);
        } else if b38 > a8c {
            wrf(a8, b38);
            let st = rd32(this.wrapping_add(THIS_STATUS));
            (this.wrapping_add(THIS_STATUS) as *mut u32)
                .write_unaligned((st & !1) | 4);
        }

        // Block 14: probe again (second site) and finalize.
        let edx: u32 = if probe(a1) == 0 {
            rd32(a1.wrapping_add(PARAM_FALLBACK))
        } else {
            let inner = probe(a1);
            let fetch: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                rd32(rd32(inner).wrapping_add(VT_SLOT_TABLE)) as usize,
            );
            fetch(inner)
        };
        let row2 = r3
            .wrapping_mul(ROW_STRIDE)
            .wrapping_add(rd32(rd32(edx.wrapping_add(4))));
        lf_checker_rt::callee_thiscall!(13, u32, edx, row2)
    }
});
