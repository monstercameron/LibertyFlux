// original: 0x009687e0 timing_field_update (proposed)

/// Resample a 3x3 neighbourhood of grid cells and accumulate four
/// direction estimates into an output array.
///
/// `a1` selects the centre cell of a 120-wide grid (`col = a1 % 120`,
/// `row = a1 / 120`); the nine neighbours `(col + dc, row + dr)` are each
/// evaluated through one intercepted grid-cell callee (ids 1-9, stdcall,
/// three arguments: column, row, out-pointer) into nine frame buffers holding
/// a 3-vector at `+0/+4/+8` and a coordinate pair at `+0x10/+0x14`. `this`
/// supplies five tuning floats, `scale` multiplies the last cell's
/// contribution, and `a3` points to four structs of stride 0x18 receiving
/// `{x, y, z, index}` each.
///
/// The reduction runs an outer loop of four with an inner loop of nine: per
/// inner step two difference vectors are formed against one running carry
/// (seeded per outer step, fed by the previous step's join value),
/// near-zero ones take a
/// constant path while the rest zero out through a NaN guard, and the pair
/// combines into two clamped factors whose product is stored; the nine
/// products normalise nine weights that scale the nine cell vectors into the
/// output struct, whose index holds a division remainder stepping by 0x5a.
/// All float operations keep the original's operand order; every
/// compare-and-branch is an exact emulation including NaN (unordered takes
/// `jb`/`jbe`, never `ja`). The return value is the last division quotient.
///
/// The grid callees, the tuning callee (id 10, cdecl, one argument, float
/// result on the x87 stack), a parameterless event callee (id 11) and the
/// security-cookie check (id 12, register-preserving) are all intercepted.
///
/// Original: thiscall, three stack words, callee cleans them.
lf_checker_rt::export!(thiscall, rw_009687e0(this: u32, a1: u32, scale: f32, a3: u32) -> u32 {
    unsafe {
        const GRID_WIDE: u32 = 120;
        const STEPS: u32 = 0x1c2;
        const REM_STEP: u32 = 0x5a;
        const DIVISOR: u32 = 360;

        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        /// Truncate exactly like `cvttss2si` (out-of-range or NaN yields MIN).
        fn cvtt(x: f32) -> i32 {
            const LIM: f32 = 2147483648.0;
            if x.is_nan() || x >= LIM || x < -LIM {
                i32::MIN
            } else {
                x as i32
            }
        }
        /// Clamp `v` to [0, 1] exactly like the original's `comiss`/`jbe`
        /// pair (NaN passes through).
        fn clamp01(v: f32) -> f32 {
            if 0.0 > v {
                0.0
            } else if v > 1.0 {
                1.0
            } else {
                v
            }
        }

        let col = a1 % GRID_WIDE;
        let row = a1 / GRID_WIDE;
        let mut buf = [[0u32; 6]; 9];
        let mut k = 0usize;
        while k < 9 {
            let dc = (k % 3) as u32;
            let dr = 2 - (k / 3) as u32;
            let c = col.wrapping_add(dc).wrapping_sub(1);
            let r = row.wrapping_add(dr).wrapping_sub(1);
            let p = buf[k].as_mut_ptr() as u32;
            let _ = match k {
                0 => lf_checker_rt::callee_stdcall!(1, u32, c, r, p),
                1 => lf_checker_rt::callee_stdcall!(2, u32, c, r, p),
                2 => lf_checker_rt::callee_stdcall!(3, u32, c, r, p),
                3 => lf_checker_rt::callee_stdcall!(4, u32, c, r, p),
                4 => lf_checker_rt::callee_stdcall!(5, u32, c, r, p),
                5 => lf_checker_rt::callee_stdcall!(6, u32, c, r, p),
                6 => lf_checker_rt::callee_stdcall!(7, u32, c, r, p),
                7 => lf_checker_rt::callee_stdcall!(8, u32, c, r, p),
                _ => lf_checker_rt::callee_stdcall!(9, u32, c, r, p),
            };
            k += 1;
        }
        let cell_f = |kk: usize, w: usize| -> f32 { f32::from_bits(buf[kk][w]) };

        let t90 = rdf(this.wrapping_add(0x2f90));
        let t64 = rdf(this.wrapping_add(0x2f64));
        let t60 = rdf(this.wrapping_add(0x2f60));
        let t68 = rdf(this.wrapping_add(0x2f68));
        let t94 = rdf(this.wrapping_add(0x2f94));
        let one = rdf(lf_checker_rt::relocated(0x00fe_88e8));
        let s_arg = add(add(mul(t64, t64), mul(t60, t60)), mul(t68, t68));
        let fs: f32 = lf_checker_rt::callee_cdecl!(10, f32, s_arg.to_bits());
        let f60 = mul(t60, fs);
        let f64 = mul(t64, fs);
        let f68 = mul(t68, fs);
        let zero = 0.0f32;
        // Q and R, the zero-multiplied terms included for bit-exactness.
        let q = add(add(mul(f60, zero), f64), mul(f68, zero));
        let cmp1 = rdf(lf_checker_rt::relocated(0x00fe_8d94));
        let mut x0: f32;
        if !(q > cmp1) {
            x0 = rdf(lf_checker_rt::relocated(0x00fe_8aa0));
        } else if !(one > q) {
            x0 = zero;
        } else {
            let _: u32 = lf_checker_rt::callee_cdecl!(11, u32,);
            x0 = q;
        }
        x0 = mul(x0, rdf(lf_checker_rt::relocated(0x00e7_c2a8)));
        let mut x1 = add(add(mul(f64, zero), f60), mul(f68, zero));
        if !(0.0 >= x1) {
            x1 = x0;
        } else {
            x1 = sub(rdf(lf_checker_rt::relocated(0x00fe_8c1c)), x0);
        }
        let tr = cvtt(x1);
        let mut v = STEPS.wrapping_sub(tr as u32);
        // Coordinate pairs of the plus-shaped neighbours.
        let p0 = cell_f(1, 4);
        let p1 = cell_f(1, 5);
        let p2 = cell_f(3, 4);
        let p3 = cell_f(3, 5);
        let p4 = cell_f(5, 4);
        let p5 = cell_f(5, 5);
        let p6 = cell_f(7, 4);
        let p7 = cell_f(7, 5);
        let pairs = [p0, p1, p2, p3, p4, p5, p6, p7];
        let eps = rdf(lf_checker_rt::relocated(0x00fe_863c));
        let cdata = || rdf(lf_checker_rt::relocated(0x0103_8834));
        let mut ret: u32 = 0;
        let mut outer = 0u32;
        while outer < 4 {
            let (pp0, pp1) = (pairs[(outer * 2) as usize], pairs[(outer * 2 + 1) as usize]);
            // xmm3 carries across inner iterations: the previous step's join
            // value, mutilated by the blend multiply (seeded with t94).
            let mut x3carry = t94;
            let mut o = [0.0f32; 9];
            let mut i = 0u32;
            while i < 9 {
                // First difference vector.
                let d7 = sub(t90, pp0);
                let d6 = sub(x3carry, pp1);
                let dd = add(mul(d6, d6), mul(d7, d7));
                let (v7, v6): (f32, f32);
                if !(eps > dd) {
                    // NaN guard: ordered input zeroes the vector.
                    if dd.is_nan() {
                        let root = dd.sqrt();
                        let inv = core::hint::black_box(one) / core::hint::black_box(root);
                        v7 = mul(d7, inv);
                        v6 = mul(d6, inv);
                    } else {
                        v7 = mul(d7, zero);
                        v6 = mul(d6, zero);
                    }
                } else {
                    v7 = 1.0;
                    v6 = 0.0;
                }
                // Second difference vector, anchored at the same carry.
                let c1 = sub(x3carry, cell_f(i as usize, 5));
                let c2 = sub(t90, cell_f(i as usize, 4));
                let ee = add(mul(c1, c1), mul(c2, c2));
                let (w3, w0, x3j): (f32, f32, f32);
                if !(eps > ee) {
                    if ee.is_nan() {
                        let root = ee.sqrt();
                        let inv = core::hint::black_box(one) / core::hint::black_box(root);
                        w3 = mul(c2, inv);
                        w0 = mul(c1, inv);
                        x3j = w3;
                    } else {
                        w3 = mul(c2, zero);
                        w0 = mul(c1, zero);
                        x3j = w3;
                    }
                } else {
                    w3 = 1.0;
                    w0 = 0.0;
                    x3j = 1.0;
                }
                let f_a = if i == 4 {
                    x3carry = x3j;
                    one
                } else {
                    x3carry = mul(x3j, v7);
                    clamp01(add(mul(w0, v6), mul(w3, v7)))
                };
                let cd = cdata();
                let ww = add(mul(mul(c1, cd), mul(c1, cd)), mul(mul(c2, cd), mul(c2, cd)));
                let inv_w = core::hint::black_box(one) / core::hint::black_box(ww);
                let f_b = clamp01(inv_w);
                o[i as usize] = mul(f_b, f_a);
                i += 1;
            }
            let mut sum = mul(o[0], o[0]);
            let mut j = 1usize;
            while j < 9 {
                sum = add(sum, mul(o[j], o[j]));
                j += 1;
            }
            let norm = core::hint::black_box(one) / core::hint::black_box(sum.sqrt());
            let w = [
                mul(o[0], norm),
                mul(o[1], norm),
                mul(o[2], norm),
                mul(o[3], norm),
                mul(o[4], norm),
                mul(o[5], norm),
                mul(o[6], norm),
                mul(o[7], norm),
                mul(o[8], norm),
            ];
            let base = a3.wrapping_add(outer.wrapping_mul(0x18));
            let mut ay = mul(cell_f(0, 1), w[0]);
            let mut az = mul(cell_f(0, 2), w[0]);
            let mut ax = mul(cell_f(0, 0), w[0]);
            let mut kk = 1usize;
            while kk < 8 {
                ay = add(ay, mul(cell_f(kk, 1), w[kk]));
                az = add(az, mul(cell_f(kk, 2), w[kk]));
                ax = add(ax, mul(cell_f(kk, 0), w[kk]));
                kk += 1;
            }
            ay = mul(add(ay, mul(cell_f(8, 1), w[8])), scale);
            az = mul(add(az, mul(cell_f(8, 2), w[8])), scale);
            ax = mul(add(ax, mul(cell_f(8, 0), w[8])), scale);
            wrf(base.wrapping_add(4), ay);
            wrf(base.wrapping_add(8), az);
            wrf(base, ax);
            let ea = v;
            v = v.wrapping_add(REM_STEP);
            ret = ea / DIVISOR;
            wrf(base.wrapping_add(0xc), f32::from_bits(ea % DIVISOR));
            outer += 1;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(12, u32, 0);
        ret
    }
});
