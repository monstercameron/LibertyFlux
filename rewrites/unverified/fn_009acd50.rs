// original: 0x009ACD50 ped_task_update_levels (proposed)

/// Update four per-task level slots from smoothed inputs, timers and tables.
///
/// `this` points to the task owner. Four object slots live at `this+0x13A4`
/// (one per channel) with a companion slot each at `this+0x1434`. `arg0` is an
/// opaque value forwarded to the level helper. Returns the leftover EAX of the
/// last executed instruction (a callee answer, a table base or zero); the sole
/// caller ignores it.
///
/// Behaviour: a time base is formed from two selector globals (each is the
/// second word unless it holds -1, an equality test, compared unsigned), the
/// first scaled by 1/60 and added to the second. Ten smoothing-helper answers
/// feed two five-entry curves (loop A); each entry is a sum clamped down to at
/// most 1.0 (a NaN sum is kept, the jump is taken on unordered) times a
/// product of three answers. The second curve's store runs after the loop
/// counter increments, so entry k lands one slot past the nominal start and
/// the last entry sits just past the array end. A direction vector is read
/// from a table chosen through the
/// thread-local block (slot index from a global, row from a word at +0x70 of
/// that block); its length normalises it, and the normalised middle component
/// selects: at or below -1 (or NaN) the angle is pi, at or above 1 it is 0,
/// otherwise the angle helper (which takes its argument in XMM0 and answers
/// in XMM0) converts it; times 180/pi and truncated to i32 (exact truncation,
/// out of range or NaN gives 0x80000000) this seeds the main loop base.
///
/// Loop B runs four channels. Each divides a table value plus the base by 360
/// (signed 32-bit division, exact; the remainder feeds the word helper later),
/// reads a per-channel factor, optionally blends it with a
/// helper answer gated by two flag bytes, then runs two halves. Each half may
/// allocate its object slot through the setup/build helpers (whose struct
/// arguments live on the original's stack and are snapshotted, not
/// address-compared) and then: forms a level input from the curves, takes the
/// level helper's answer through a log10-style block (mantissa/exponent split,
/// polynomial in mantissa, 20*log10 scaled, -100 floor for inputs at most
/// 1e-6 apart, NaN included), stores it to the object's +0x80 (first half) or
/// keeps it (second half), resolves an output row through the stride/table
/// globals (a 0xFF tag byte selects row 0, an equality test), and calls the
/// float/int/word output helpers with the level, a truncated 64-bit conversion
/// of the magnitude answer (exact truncation, NaN or out of range gives
/// 0x8000000000000000, only the low 32 bits are passed on) and the sign word.
/// The tail scales four accumulators by 0.25 (when its flag byte is set) and
/// pushes them through the float/int helpers for the stored objects.
///
/// All integer arithmetic wraps; all float arithmetic is single precision in
/// the original's operand order with pinned evaluation order; float
/// comparisons use the original's unordered (NaN) outcomes. Compared values:
/// the -1 selectors and flag bytes and tag byte use equality (signedness
/// immaterial); the idiv dividend/divisor are signed; loop counters count up
/// to fixed trip counts; every comiss uses the NaN-exact form of its jump.
///
/// The snapshot word just before each setup/build struct argument is the
/// second curve's last entry, fully determined, not uninitialised memory.
///
/// Original: 0x009ACD50 (thiscall, one stack word, callee cleanup).
lf_checker_rt::export!(thiscall, rw_009ACD50(this: u32, arg0: u32) -> u32 {
    unsafe {
        const C_SMOOTH: u32 = 1;
        const C_NORM: u32 = 2;
        const C_ANGLE: u32 = 3;
        const C_BLEND: u32 = 4;
        const C_SETUP: u32 = 5;
        const C_BUILD: u32 = 6;
        const C_LEVEL: u32 = 7;
        const C_SETF: u32 = 8;
        const C_MAG: u32 = 9;
        const C_SETI: u32 = 10;
        const C_SETW: u32 = 11;
        const C_COOKIE: u32 = 12;

        const G_TIME_A2: u32 = 0x1295858;
        const G_TIME_A1: u32 = 0x129584C;
        const G_TIME_B2: u32 = 0x1295854;
        const G_TIME_B1: u32 = 0x1295848;
        const G_F1: u32 = 0x1168824;
        const G_TLS_SLOT: u32 = 0x17ABA14;
        const G_F2: u32 = 0x12845F0;
        const G_FB: u32 = 0x1167FC0;
        const G_FA_BASE: u32 = 0x11681BC;
        const G_FC_BASE: u32 = 0x1168750;
        const G_GATE1: u32 = 0x1168A5C;
        const G_GATE2: u32 = 0x103909C;
        const G_F3: u32 = 0x116828C;
        const G_F4: u32 = 0x1292444;
        const G_STRIDE: u32 = 0x115D968;
        const G_TABLE: u32 = 0x115D988;
        const G_FD_BASE: u32 = 0x1165C9C;
        const G_TAILGATE_W: u32 = 0x1283048;
        const G_VEC_BASE: u32 = 0x115E3F0;
        const G_COOKIE: u32 = 0x1057FB4;
        const BLEND_THIS: u32 = 0x1165880;
        const TMPL_A: u32 = 0xE923C8;
        const TMPL_B: u32 = 0xE92404;
        const SMOOTH_A: u32 = 0x128A8DC;
        const SMOOTH_B: u32 = 0x12891E0;
        const SMOOTH_C: u32 = 0x128918C;
        const SMOOTH_D: u32 = 0x1289208;

        const K_1_60: f32 = f32::from_bits(0x3C888889);
        const K_1E_6: f32 = f32::from_bits(0x3727C5AC);
        const K_127: f32 = f32::from_bits(0x42FE0000);
        const K_POLY_A: f32 = f32::from_bits(0x40019460);
        const K_POLY_B: f32 = f32::from_bits(0x3FD6633D);
        const K_POLY_C: f32 = f32::from_bits(0x3EB08FDF);
        const K_LOG10_2: f32 = f32::from_bits(0x3E9A209B);
        const K_20: f32 = f32::from_bits(0x41A00000);
        const K_NEG100: f32 = f32::from_bits(0xC2C80000);
        const K_PI: f32 = f32::from_bits(0x40490FDB);
        const K_180_PI: f32 = f32::from_bits(0x42652EE0);
        const K_360: f32 = f32::from_bits(0x43B40000);
        const K_QTR: f32 = f32::from_bits(0x3E800000);
        const K_I32_MAX_P1: f32 = f32::from_bits(0x4F000000);
        const K_I32_MIN: f32 = f32::from_bits(0xCF000000);
        const K_I64_MAX_P1: f32 = f32::from_bits(0x5F000000);
        const K_I64_MIN: f32 = f32::from_bits(0xDF000000);
        const ONE: f32 = 1.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        fn glob(file_va: u32) -> u32 {
            lf_checker_rt::relocated(file_va)
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
        /// comiss a,b followed by jbe (taken when a<=b or unordered).
        #[inline(always)]
        fn jbe_f32(a: f32, b: f32) -> bool {
            !(a > b)
        }
        /// comiss a,b followed by jb (taken when a<b or unordered).
        #[inline(always)]
        fn jb_f32(a: f32, b: f32) -> bool {
            !(b <= a)
        }
        /// Exact cvttss2si: truncate toward zero, NaN or out of i32 range
        /// gives 0x80000000.
        #[inline(always)]
        fn cvtt_exact(x: f32) -> i32 {
            if x.is_nan() || x >= K_I32_MAX_P1 || x < K_I32_MIN {
                i32::MIN
            } else {
                x as i32
            }
        }
        /// Exact fistp-with-chop of an f32 to i64: truncate toward zero, NaN
        /// or out of i64 range gives 0x8000000000000000.
        #[inline(always)]
        fn fistp_chop(x: f32) -> i64 {
            if x.is_nan() || x >= K_I64_MAX_P1 || x < K_I64_MIN {
                i64::MIN
            } else {
                x as i64
            }
        }
        /// Output-row lookup: tag byte 0xFF selects row 0, else
        /// stride*tag + table[row*0x6F40 + 0x6F14], all wrapping.
        #[inline(always)]
        unsafe fn lookup(obj: u32, stride: u32, table: u32) -> u32 {
            unsafe {
                let b4 = rd8(obj.wrapping_add(4)) as u32;
                if b4 == 0xFF {
                    0
                } else {
                    let b40 = rd8(obj.wrapping_add(0x40)) as u32;
                    let row = b40.wrapping_mul(0x6F40);
                    stride.wrapping_mul(b4).wrapping_add(rd32(
                        table.wrapping_add(row).wrapping_add(0x6F14),
                    ))
                }
            }
        }
        /// Level block: mantissa/exponent split of the answer, polynomial in
        /// the mantissa, 20*log10 scaled, -100 floor. Bit-exact.
        #[inline(always)]
        fn log_level(ans: f32) -> f32 {
            let bits = ans.to_bits();
            let exp = bits >> 23;
            let x1 = sub(ans, K_1E_6);
            let xd = core::hint::black_box(exp as f64) + core::hint::black_box(0.0);
            let mant = (bits & 0x7FFFFF) | 0x3F800000;
            let under = jb_f32(x1, 0.0);
            let mut x0f = sub(xd as f32, K_127);
            let mantf = f32::from_bits(mant);
            let mut x2 = mul(mantf, K_POLY_A);
            x0f = sub(x0f, K_POLY_B);
            x2 = add(x2, x0f);
            if !under {
                let mut x3 = mul(mantf, mantf);
                x3 = mul(x3, K_POLY_C);
                x2 = sub(x2, x3);
                x2 = mul(x2, K_LOG10_2);
                x2 = mul(x2, K_20);
                x2
            } else {
                K_NEG100
            }
        }

        // Tracks EAX; only the exit value is compared (ret:eax).
        let mut eaxv: u32 = 0;

        // Time base from the selector globals.
        let g_a = rd32(glob(G_TIME_A2));
        let sel_a = if g_a != 0xFFFFFFFF {
            g_a
        } else {
            rd32(glob(G_TIME_A1))
        };
        let g_b = rd32(glob(G_TIME_B2));
        let sel_b = if g_b != 0xFFFFFFFF {
            g_b
        } else {
            rd32(glob(G_TIME_B1))
        };
        let v0 = add(mul((sel_a as i32) as f32, K_1_60), (sel_b as i32) as f32);

        // Prologue smoothing answers.
        let a1: f32 = lf_checker_rt::callee_thiscall!(
            C_SMOOTH, f32, this.wrapping_add(0x1504), v0.to_bits()
        );
        eaxv = a1.to_bits();
        let a2: f32 = lf_checker_rt::callee_thiscall!(
            C_SMOOTH, f32, this.wrapping_add(0x157C), v0.to_bits()
        );
        eaxv = a2.to_bits();
        let g_f1 = rdf(glob(G_F1));
        let a3: f32 = lf_checker_rt::callee_thiscall!(
            C_SMOOTH, f32, glob(SMOOTH_A), g_f1.to_bits()
        );
        eaxv = a3.to_bits();
        let a4: f32 = lf_checker_rt::callee_thiscall!(
            C_SMOOTH, f32, glob(SMOOTH_B), g_f1.to_bits()
        );
        eaxv = a4.to_bits();
        // Thread-local vector table.
        let tls_idx = rd32(glob(G_TLS_SLOT));
        let tls_ptr = lf_checker_rt::tls_slot(tls_idx as usize);
        eaxv = tls_ptr;
        eaxv = rd32(eaxv.wrapping_add(0x70));
        let vec_base = eaxv.wrapping_shl(6).wrapping_add(glob(G_VEC_BASE));
        eaxv = vec_base;
        let x5 = rdf(vec_base.wrapping_add(0x38));
        let a5: f32 = lf_checker_rt::callee_thiscall!(
            C_SMOOTH, f32, glob(SMOOTH_C), x5.to_bits()
        );
        eaxv = a5.to_bits();
        let g_f2 = rdf(glob(G_F2));
        let a6: f32 = lf_checker_rt::callee_thiscall!(
            C_SMOOTH, f32, glob(SMOOTH_D), g_f2.to_bits()
        );
        eaxv = a6.to_bits();
        let p1 = mul(a6, a3);
        let s1c = mul(p1, a1);
        let s50 = mul(p1, a2);
        let lane_this = this.wrapping_add(0x14B4);

        // Loop A: the two five-entry curves. The first sits at arr[0..5];
        // the second lands one slot past its nominal start, arr[5..10].
        let mut arr = [0.0f32; 10];
        for esi in 0..5u32 {
            let ga = if esi == 4 {
                rdf(glob(G_FB))
            } else {
                rdf(glob(G_FA_BASE.wrapping_add(esi.wrapping_mul(4))))
            };
            let gc = rdf(glob(G_FC_BASE.wrapping_add(esi.wrapping_mul(4))));
            let t1: f32 = lf_checker_rt::callee_thiscall!(
                C_SMOOTH, f32, lane_this, gc.to_bits()
            );
            eaxv = t1.to_bits();
            let t2: f32 = lf_checker_rt::callee_thiscall!(
                C_SMOOTH, f32, this.wrapping_add(0x14DC), ga.to_bits()
            );
            eaxv = t2.to_bits();
            let mut s = add(t1, t2);
            if s > ONE {
                s = ONE;
            }
            arr[esi as usize] = mul(s, s1c);
            let gb = if esi == 4 {
                rdf(glob(G_FB))
            } else {
                rdf(glob(G_FA_BASE.wrapping_add(esi.wrapping_mul(4))))
            };
            let t3: f32 = lf_checker_rt::callee_thiscall!(
                C_SMOOTH, f32, this.wrapping_add(0x152C), gc.to_bits()
            );
            eaxv = t3.to_bits();
            let t4: f32 = lf_checker_rt::callee_thiscall!(
                C_SMOOTH, f32, this.wrapping_add(0x1554), gb.to_bits()
            );
            eaxv = t4.to_bits();
            let mut s2 = add(t3, t4);
            if s2 > ONE {
                s2 = ONE;
            }
            // Stored after the counter increments: entry k lands at k+1.
            arr[esi as usize + 5] = mul(s2, s50);
        }

        // Direction vector, normalise, angle to degrees, truncate.
        eaxv = rd32(tls_ptr.wrapping_add(0x70));
        let vec_base2 = eaxv.wrapping_shl(6).wrapping_add(glob(G_VEC_BASE));
        eaxv = vec_base2;
        let vx = rdf(vec_base2.wrapping_add(4));
        let vy = rdf(vec_base2);
        let vz = rdf(vec_base2.wrapping_add(8));
        let q = add(add(mul(vx, vx), mul(vy, vy)), mul(vz, vz));
        let s: f32 = lf_checker_rt::callee_cdecl!(C_NORM, f32, q.to_bits());
        eaxv = s.to_bits();
        // Note the cross-over: the [eax+4] component is scaled into y1 and
        // the [eax] component into x1 (the stores cross the slots).
        let x1 = mul(vy, s);
        let y1 = mul(vx, s);
        let z1 = mul(vz, s);
        let z0 = mul(z1, 0.0);
        let x0 = mul(x1, 0.0);
        let v = add(add(x0, y1), z0);
        let ang = if jbe_f32(v, f32::from_bits(0xBF800000)) {
            K_PI
        } else if jbe_f32(ONE, v) {
            0.0
        } else {
            let a: u32 = lf_checker_rt::callee_cdecl!(C_ANGLE, u32, v.to_bits());
            eaxv = a;
            f32::from_bits(a)
        };
        let deg = mul(ang, K_180_PI);
        // Steering value: ((y1*0.0)+x1)+z1. Positive or NaN keeps the
        // degrees; otherwise the truncated value is 360.0 minus degrees.
        let cond = add(add(mul(y1, 0.0), x1), z1);
        let n_input = if jb_f32(0.0, cond) {
            deg
        } else {
            sub(K_360, deg)
        };
        let n = cvtt_exact(n_input);
        eaxv = n as u32;
        let loop_base = 0x1C2u32.wrapping_sub(n as u32);

        // Loop B over the four channels.
        let stride = rd32(glob(G_STRIDE));
        let table = rd32(glob(G_TABLE));
        let mut slot292 = this.wrapping_add(0x1444);
        eaxv = slot292;
        eaxv = glob(G_FD_BASE).wrapping_sub(this);
        let fd_off = eaxv;
        let div_tbl = [0u32, 0x5A, 0xB4, 0x10E];
        let mut slot308 = 0.0f32;
        let mut slot316f = 0.0f32;
        let mut slot264 = 0.0f32;
        let mut slot248 = 0.0f32;
        let mut counter = 1u32;
        let mut edi_idx = 0u32;
        let mut xmm3v = ONE;
        // High byte of the x87 control word left by fnstcw (0 until the
        // first half runs one); it overlaps the blend byte-argument word.
        let mut cw_hi: u32 = 0;
        let mut xmm2v = 0.0f32;
        let mut xmm4v = 0.0f32;
        let mut esi = this.wrapping_add(0x13A4);
        for k in 0..4u32 {
            let e = (div_tbl[(edi_idx >> 2) as usize].wrapping_add(loop_base)) as i32;
            let q0 = e.wrapping_div(360);
            let rem_flag = e.wrapping_rem(360) as u32;
            eaxv = q0 as u32;
            eaxv = rd32(esi.wrapping_add(fd_off));
            let f1 = f32::from_bits(eaxv);
            let mut slot312 = f1;
            let mut xmm1 = f1;
            let gate = rd8(glob(G_GATE1)) != 0 && rd8(glob(G_GATE2)) != 0;
            if gate {
                let mut w1 = 0u32;
                // The byte argument sits two bytes before the trip counter and
                // one byte into the control-word slot: low byte is the zero
                // init, then the control high byte, then the trip counter.
                let arg1word: u32 = (cw_hi << 8) | ((4 - k) << 16);
                let ans4: u32 = lf_checker_rt::callee_thiscall!(
                    C_BLEND, u32, glob(BLEND_THIS),
                    (&arg1word as *const u32) as u32,
                    (&mut w1 as *mut u32) as u32
                );
                eaxv = ans4;
                let w1f = f32::from_bits(w1);
                let t_a = sub(ONE, w1f);
                let t_b = sub(ONE, rdf(glob(G_F3)));
                xmm1 = add(mul(t_a, f1), mul(t_b, w1f));
                xmm3v = ONE;
            }
            let t_c = sub(xmm3v, rdf(glob(G_F4)));
            slot312 = mul(t_c, xmm1);
            let pre = rd32(esi);
            let mut run_first = pre != 0;
            if !run_first {
                let mut s2 = [0u32; 18];
                s2[14] = 0xFFFFFFFF;
                s2[15] = 0x46BAB800;
                s2[16] = 0xFFFFFFFF;
                s2[17] = 0xFF;
                let mut s76 = [0u32; 4];
                s76[0] = arr[9].to_bits();
                let s76ptr = (&mut s76[1] as *mut u32) as u32;
                let ans5: u32 = lf_checker_rt::callee_cdecl!(
                    C_SETUP, u32, s76ptr, 0x40, glob(TMPL_A), counter
                );
                eaxv = ans5;
                let ans6: u32 = lf_checker_rt::callee_thiscall!(
                    C_BUILD, u32, this, s76ptr, esi, s2.as_ptr() as u32,
                    0xFFFFFFFF, 0, 0
                );
                eaxv = ans6;
                if rd32(esi) != 0 {
                    xmm3v = ONE;
                    run_first = true;
                }
            }
            if run_first {
                let mut t = mul(add(arr[(edi_idx >> 2) as usize], arr[4]), slot312);
                if t > xmm3v {
                    t = xmm3v;
                }
                let this2a = slot292.wrapping_sub(0x90);
                let ans7: f32 = lf_checker_rt::callee_thiscall!(
                    C_LEVEL, f32, this2a, t.to_bits(), arg0
                );
                eaxv = ans7.to_bits();
                let lvl1 = log_level(ans7);
                wrf(esi.wrapping_add(0x80), lvl1);
                eaxv = rd32(esi);
                let edx = lookup(eaxv, stride, table);
                let ans8: u32 = lf_checker_rt::callee_thiscall!(
                    C_SETF, u32, edx, lvl1.to_bits()
                );
                eaxv = ans8;
                let ans9: f32 =
                    lf_checker_rt::callee_cdecl!(C_MAG, f32, a4.to_bits(), a5.to_bits());
                eaxv = ans9.to_bits();
                let slot252 = ans9;
                eaxv = rd32(esi);
                let edx2 = lookup(eaxv, stride, table);
                let conv1 = fistp_chop(ans9);
                cw_hi = 0x03;
                let ans10: u32 = lf_checker_rt::callee_thiscall!(
                    C_SETI, u32, edx2, conv1 as u32
                );
                eaxv = ans10;
                eaxv = rd32(esi);
                let edx3 = lookup(eaxv, stride, table);
                let ans11: u32 = lf_checker_rt::callee_thiscall!(
                    C_SETW, u32, edx3, rem_flag
                );
                eaxv = ans11;
                slot308 = add(rdf(esi.wrapping_add(0x80)), slot308);
                slot316f = add(slot252, slot316f);
            }
            let edi_ptr = esi.wrapping_add(0x90);
            let mut run_second = rd32(edi_ptr) != 0;
            if !run_second {
                let mut s2b = [0u32; 18];
                s2b[14] = 0xFFFFFFFF;
                s2b[15] = 0x46BAB800;
                s2b[16] = 0xFFFFFFFF;
                s2b[17] = 0xFF;
                let mut s76b = [0u32; 4];
                s76b[0] = arr[9].to_bits();
                let s76bptr = (&mut s76b[1] as *mut u32) as u32;
                let ans5b: u32 = lf_checker_rt::callee_cdecl!(
                    C_SETUP, u32, s76bptr, 0x20, glob(TMPL_B), counter
                );
                eaxv = ans5b;
                let ans6b: u32 = lf_checker_rt::callee_thiscall!(
                    C_BUILD, u32, this, s76bptr, edi_ptr, s2b.as_ptr() as u32,
                    0xFFFFFFFF, 0, 0
                );
                eaxv = ans6b;
                if rd32(edi_ptr) == 0 {
                    xmm2v = slot264;
                    xmm4v = slot248;
                } else {
                    run_second = true;
                }
            }
            if run_second {
                // The fixed addend is the second curve's last entry.
                let mut u = mul(
                    add(arr[5 + (edi_idx >> 2) as usize], arr[9]),
                    slot312,
                );
                if u > ONE {
                    u = ONE;
                }
                let ans7b: f32 = lf_checker_rt::callee_thiscall!(
                    C_LEVEL, f32, slot292, u.to_bits(), arg0
                );
                eaxv = ans7b.to_bits();
                let lvl2 = log_level(ans7b);
                slot312 = lvl2;
                eaxv = rd32(edi_ptr);
                let edx = lookup(eaxv, stride, table);
                let ans8: u32 = lf_checker_rt::callee_thiscall!(
                    C_SETF, u32, edx, lvl2.to_bits()
                );
                eaxv = ans8;
                let ans9b: f32 =
                    lf_checker_rt::callee_cdecl!(C_MAG, f32, a4.to_bits(), a5.to_bits());
                eaxv = ans9b.to_bits();
                let slot252b = ans9b;
                eaxv = rd32(edi_ptr);
                let edx2 = lookup(eaxv, stride, table);
                let conv = fistp_chop(ans9b);
                cw_hi = 0x03;
                let ans10: u32 = lf_checker_rt::callee_thiscall!(
                    C_SETI, u32, edx2, conv as u32
                );
                eaxv = ans10;
                eaxv = rd32(edi_ptr);
                let edx3 = lookup(eaxv, stride, table);
                let ans11: u32 = lf_checker_rt::callee_thiscall!(
                    C_SETW, u32, edx3, rem_flag
                );
                eaxv = ans11;
                xmm2v = add(slot312, slot264);
                slot264 = xmm2v;
                xmm4v = add(slot252b, slot248);
                slot248 = xmm4v;
            }
            slot292 = slot292.wrapping_add(0x1C);
            xmm3v = ONE;
            counter = counter.wrapping_add(1);
            edi_idx = edi_idx.wrapping_add(4);
            esi = esi.wrapping_add(4);
            let _ = k;
        }

        // Tail: scale the accumulators and push them out.
        let tailgate = rd8(glob(G_TAILGATE_W).wrapping_add(1));
        if tailgate != 0 {
            let s308 = mul(slot308, K_QTR);
            let s276 = mul(slot316f, K_QTR);
            let s264 = mul(xmm2v, K_QTR);
            let s228 = mul(xmm4v, K_QTR);
            xmm2v = s264;
            let mut tsi = this.wrapping_add(0x1434);
            for _ in 0..4u32 {
                eaxv = rd32(tsi.wrapping_sub(0x90));
                if eaxv != 0 {
                    let edx = lookup(eaxv, stride, table);
                    let a: u32 = lf_checker_rt::callee_thiscall!(
                        C_SETF, u32, edx, s308.to_bits()
                    );
                    eaxv = a;
                    eaxv = rd32(tsi.wrapping_sub(0x90));
                    let edx2 = lookup(eaxv, stride, table);
                    let conv = fistp_chop(s276);
                    let b: u32 = lf_checker_rt::callee_thiscall!(
                        C_SETI, u32, edx2, conv as u32
                    );
                    eaxv = b;
                    xmm2v = s264;
                }
                eaxv = rd32(tsi);
                if eaxv != 0 {
                    let edx = lookup(eaxv, stride, table);
                    let c: u32 = lf_checker_rt::callee_thiscall!(
                        C_SETF, u32, edx, xmm2v.to_bits()
                    );
                    eaxv = c;
                    eaxv = rd32(tsi);
                    let edx2 = lookup(eaxv, stride, table);
                    let conv = fistp_chop(s228);
                    let d: u32 = lf_checker_rt::callee_thiscall!(
                        C_SETI, u32, edx2, conv as u32
                    );
                    eaxv = d;
                }
                xmm2v = s264;
                tsi = tsi.wrapping_add(4);
            }
        }
        let cookie = rd32(glob(G_COOKIE));
        let _: u32 = lf_checker_rt::callee_thiscall!(C_COOKIE, u32, cookie);
        eaxv
    }
});
