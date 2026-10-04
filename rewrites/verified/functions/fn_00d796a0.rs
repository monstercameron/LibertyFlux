// original: 0x00d796a0 grid_solver_accumulate (proposed)

/// Solve one accumulation pass over a 2-D index grid and return a float score.
///
/// `obj` carries the inputs: a virtual table whose slot at `+0xEC` answers a
/// float pair per call, a center point (two floats at `[obj+0x20]+0x30/0x34`)
/// and a signed handle at `+0x48`. `a1`/`a5` are opaque words forwarded to the
/// row workers, `a2` is a base angle, `a4` scales the search radius, the low
/// bytes of `a6`/`a7` gate the second and third row workers, and `objb`
/// (`+0x14` limit base, `+0x28` state byte) is a small control block.
///
/// Algorithm: four virtual calls fetch float pairs whose products seed a
/// radius (`max(2, sqrt(acc)*K1+1) * 18 * a4`). The center plus/minus the
/// radius is scaled and rounded to four grid bounds (two clamped at zero from
/// below, two at `0x77` from above). A global 16-bit epoch is bumped, or
/// reset to 1 through a hook call when saturated. An outer loop (at most four
/// passes, exiting early when the two accumulator slots stop changing) looks
/// the handle up in a pool, runs the row workers over the grid slice, and
/// repeats; the inner loops walk rows taken 16 at a time from a global table.
/// The tail wraps the base angle into [-pi, pi] (also written back to the
/// caller's `a2` slot, which this rewrite cannot store to), folds the
/// accumulators against it, clears the control block's state byte when its
/// limit trips, and returns either the wrapped angle or one of the
/// accumulators. Float operation order is the original's; comparisons after
/// `comiss`/`ucomiss` keep NaN semantics (`jbe`/`jb` taken on unordered).
///
/// Original: 0x00d796a0 (cdecl, nine stack words; the fourth is not read;
/// float result in ST0; true size 1886 bytes, one past the listed 1875).
lf_checker_rt::export!(cdecl, rw_00d796a0(
    obj: u32, a1: u32, a2: u32, _a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, objb: u32,
) -> f32 {
    unsafe {
        const VT_SLOT: u32 = 0xEC;
        const K_SQRT: f32 = f32::from_bits(0x3D2A_AAAB);
        const K_RAD: f32 = f32::from_bits(0x4190_0000); // 18.0
        const C6: f32 = f32::from_bits(0x3CA3_D70A);
        const C5: f32 = f32::from_bits(0x4270_0000); // 60.0
        const BIG: f32 = f32::from_bits(0x4B00_0000);
        const BIG_BITS: u32 = 0x4B00_0000;
        const SIGN: u32 = 0x8000_0000;
        const ONE: f32 = 1.0;
        const TWO: f32 = 2.0;
        const HALF: f32 = 0.5;
        const THRESH: f32 = f32::from_bits(0xC61C_3F9A);
        const RET_CMP: f32 = f32::from_bits(0x3FC9_0FDB); // pi/2
        const PI: f32 = f32::from_bits(0x4049_0FDB);
        const TWO_PI: f32 = f32::from_bits(0x40C9_0FDB);
        const NEG_PI: f32 = f32::from_bits(0xC049_0FDB);
        const COUNT_WORD: u32 = 0x011A_8908;
        const TABLE: u32 = 0x011A_8918;
        const POOL_PTR: u32 = 0x012F_B214;
        const LIMIT: u32 = 0x0117_35B4;
        const ID_HOOK: u32 = 2;
        const ID_POOL: u32 = 3;
        const ID_PREP: u32 = 4;
        const ID_FILL: u32 = 5;
        const ID_ROW0: u32 = 6;
        const ID_ROW1: u32 = 7;
        const ID_ROW2: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        /// x86 CVTTSS2SI: 0x80000000 on NaN or out of range, else truncate.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x8000_0000u32 as i32
            } else {
                x as i32
            }
        }
        /// One grid-bound rounding block: scale `c`, round to int through the
        /// big-magic addition trick with the greater-than-sign correction.
        /// Returns the converted int and the magic addend (the outer loop's
        /// first comparison reads the second block's addend).
        #[inline(always)]
        fn round_block(c: f32) -> (i32, f32) {
            {
                let v = add(mul(c, C6), C5);
                let vb = v.to_bits();
                let sign = vb & SIGN;
                let mag = f32::from_bits(vb ^ sign);
                let below = mag < BIG;
                let b = f32::from_bits((if below { BIG_BITS } else { 0 }) | sign);
                let mut t = add(v, b);
                t = sub(t, b);
                let d = sub(t, v);
                let s = f32::from_bits(sign);
                let corr = if d > s || d.is_nan() { ONE } else { 0.0 };
                t = sub(t, corr);
                (cvtt(t), b)
            }
        }

        // Entry bytes (written by the row-0 worker, read at the tail).
        let mut flags_word: u32 = 0;
        let flags_ptr = (&mut flags_word as *mut u32) as u32;
        // Scratch slot shared by the prep/fill/row workers.
        let mut slot124: u32 = 0;
        let slot_ptr = (&mut slot124 as *mut u32) as u32;
        let a2f = f32::from_bits(a2);
        // Accumulator slots start as copies of the base angle (row workers
        // overwrite them every pass).
        let mut f4slot: f32 = a2f;
        let mut f5slot: f32 = a2f;
        let a4f = f32::from_bits(a4);
        let flag_b = (a6 & 0xFF) as u8;
        let flag_c = (a7 & 0xFF) as u8;

        // Four virtual calls; the returned pairs seed the radius.
        let vt = rd32(obj);
        let vcall: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vt + VT_SLOT) as usize);
        let mut vo0: u32 = 0;
        let mut vo1: u32 = 0;
        let mut vo2: u32 = 0;
        let mut vo3: u32 = 0;
        let r0 = vcall(obj, (&mut vo0 as *mut u32) as u32);
        let r1 = vcall(obj, (&mut vo1 as *mut u32) as u32);
        let m1 = mul(rdf(r1.wrapping_add(4)), rdf(r0.wrapping_add(4)));
        let r2 = vcall(obj, (&mut vo2 as *mut u32) as u32);
        let r3 = vcall(obj, (&mut vo3 as *mut u32) as u32);
        let m2 = mul(rdf(r2), rdf(r3));
        let acc = add(m2, m1);

        let mut rad = acc.sqrt();
        rad = mul(rad, K_SQRT);
        rad = add(rad, ONE);
        if !(TWO > rad) {
            rad = TWO;
        }
        rad = mul(rad, K_RAD);
        rad = mul(rad, a4f);

        let center = rd32(obj.wrapping_add(0x20));
        let cx = rdf(center.wrapping_add(0x30));
// continued in next edit
        let cy = rdf(center.wrapping_add(0x34));
        let box_xp = add(cx, rad);
        let box_xm = sub(cx, rad);
        let box_yp = add(cy, rad);
        let box_ym = sub(cy, rad);
        let (t1, _b1) = round_block(box_xm);
        let (t2, b2) = round_block(box_ym);
        let (t3, _b3) = round_block(box_xp);
        let (t4, _b4) = round_block(box_yp);
        let n1 = if t1 > 0 { t1 } else { 0 };
        let n2 = if t2 > 0 { t2 } else { 0 };
        let n3 = if t3 < 0x77 { t3 } else { 0x77 };
        let n4 = if t4 < 0x77 { t4 } else { 0x77 };

        // Epoch counter: bump, or reset through the hook when saturated.
        let countp = lf_checker_rt::global::<u16>(COUNT_WORD);
        let c = countp.read_unaligned();
        if c >= 0xFFFF {
            let _: u32 = lf_checker_rt::callee_cdecl!(ID_HOOK, u32,);
            countp.write_unaligned(1);
        } else {
            countp.write_unaligned(c.wrapping_add(1));
        }

        let table = lf_checker_rt::relocated(TABLE);
        let mut outer: i32 = 0;
        let mut x0 = THRESH;
        let mut x1 = b2;
        'outer: loop {
            // Ordered-equality fixpoint check (NaN falls into the body).
            if x1 == f5slot && x0 == f4slot {
                break 'outer;
            }
            if outer >= 4 {
                break 'outer;
            }
            outer += 1;
            let h = rd32(obj.wrapping_add(0x48)) as i32;
            let old88 = f5slot;
            let old84 = f4slot;
            x1 = f5slot;
            x0 = f4slot;
            if h >= 0 {
                let pool = rd32(lf_checker_rt::relocated(POOL_PTR));
                let r: u32 = lf_checker_rt::callee_thiscall!(ID_POOL, u32, pool, h as u32);
                if r != 0 {
                    slot124 = 0;
                    let _: u32 = lf_checker_rt::callee_thiscall!(ID_PREP, u32, slot_ptr);
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(ID_FILL, u32, r, slot_ptr, 4);
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        ID_ROW0, u32, slot_ptr, obj, a1,
                        box_xm.to_bits(), box_ym.to_bits(), box_xp.to_bits(), box_yp.to_bits(),
                        (&mut f5slot as *mut f32) as u32,
                        (&mut f4slot as *mut f32) as u32,
                        a5, flags_ptr, flags_ptr.wrapping_add(1), flags_ptr.wrapping_add(2)
                    );
                    if flag_b != 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(ID_PREP, u32, slot_ptr);
                        let _: u32 =
                            lf_checker_rt::callee_thiscall!(ID_FILL, u32, r, slot_ptr, 8);
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            ID_ROW1, u32, slot_ptr, obj, a1,
                            box_xm.to_bits(), box_ym.to_bits(), box_xp.to_bits(), box_yp.to_bits(),
                            (&mut f5slot as *mut f32) as u32,
                            (&mut f4slot as *mut f32) as u32
                        );
                    }
                    let _: u32 = lf_checker_rt::callee_thiscall!(ID_PREP, u32, slot_ptr);
                    x0 = old84;
                    x1 = old88;
                    continue 'outer;
                } else {
                    x0 = old84;
                    x1 = old88;
                }
            }
            // Grid loops.
            let mut inner = n2;
            if inner > n4 {
                continue 'outer;
            }
            loop {
                if n1 <= n3 {
                    slot124 = ((inner & 0xF) << 4) as u32;
                    let mut esi = n1;
                    loop {
                        let idx = ((((esi & 0xF) + ((inner & 0xF) << 4)) * 5) as u32)
                            .wrapping_mul(4);
                        let row = table.wrapping_add(idx);
                        let _: u32 = lf_checker_rt::callee_cdecl!(
                            ID_ROW0, u32, row, obj, a1,
                            box_xm.to_bits(), box_ym.to_bits(), box_xp.to_bits(), box_yp.to_bits(),
                            (&mut f5slot as *mut f32) as u32,
                            (&mut f4slot as *mut f32) as u32,
                            a5, flags_ptr, flags_ptr.wrapping_add(1), flags_ptr.wrapping_add(2)
                        );
                        if flag_b != 0 {
                            let _: u32 = lf_checker_rt::callee_cdecl!(
                                ID_ROW1, u32, row.wrapping_add(4), obj, a1,
                                box_xm.to_bits(), box_ym.to_bits(), box_xp.to_bits(),
                                box_yp.to_bits(),
                                (&mut f5slot as *mut f32) as u32,
                                (&mut f4slot as *mut f32) as u32
                            );
                        }
                        if flag_c != 0 {
                            let _: u32 = lf_checker_rt::callee_cdecl!(
                                ID_ROW2, u32, row.wrapping_add(8), obj,
                                box_xm.to_bits(), box_ym.to_bits(), box_xp.to_bits(),
                                box_yp.to_bits(),
                                (&mut f5slot as *mut f32) as u32,
                                (&mut f4slot as *mut f32) as u32
                            );
                        }
                        esi += 1;
                        if esi > n3 {
                            break;
                        }
                    }
                }
                inner += 1;
                if inner > n4 {
                    break;
                }
            }
            x0 = old84;
            x1 = old88;
        }

        // Tail: wrap the base angle, fold the accumulators against it.
        let mut w = a2f;
        while NEG_PI > w {
            w = add(w, TWO_PI);
        }
        while w > PI {
            w = sub(w, TWO_PI);
        }
        let mut t0 = sub(f5slot, w);
        let mut t1 = sub(f4slot, w);
        while NEG_PI > t0 {
            t0 = add(t0, TWO_PI);
        }
        while t0 > PI {
            t0 = sub(t0, TWO_PI);
        }
        if t0 < 0.0 {
            t0 = f32::from_bits(t0.to_bits() & !SIGN);
        }
        while NEG_PI > t1 {
            t1 = add(t1, TWO_PI);
        }
        while t1 > PI {
            t1 = sub(t1, TWO_PI);
        }
        if t1 < 0.0 {
            t1 = f32::from_bits(t1.to_bits() & !SIGN);
        }
        if f5slot == f4slot {
            if rd8(objb.wrapping_add(0x28)) == 7 {
                let lim = rd32(objb.wrapping_add(0x14)).wrapping_add(0x5DC);
                if rd32(lf_checker_rt::relocated(LIMIT)) > lim {
                    (objb.wrapping_add(0x28) as *mut u8).write(0);
                }
            }
        }
        let mut rf4 = ONE;
        let mut rf2 = ONE;
        if (flags_word & 0xFF) as u8 != 0 {
            rf4 = HALF;
        }
        if ((flags_word >> 8) & 0xFF) as u8 != 0 {
            rf2 = HALF;
        }
        if t0 > RET_CMP && t1 > RET_CMP {
            return w;
        }
        let p4 = mul(rf4, t0);
        let p2 = mul(rf2, t1);
        if p2 > p4 {
            f5slot
        } else {
            f4slot
        }
    }
});
