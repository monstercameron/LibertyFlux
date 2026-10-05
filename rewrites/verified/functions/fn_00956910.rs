// original: 0x00956910 sched_time_pump (proposed)

/// Advance the ped task scheduler's time base by one pump and dispatch the
/// pending task work.
///
/// `arg0` (low byte) selects the mode. When it is non-zero the function runs
/// a single pass: it resets the pending count, asks the task dispatcher
/// (callee 2) and the time keeper (callee 4) to run, mirrors a recognised
/// entity record into the scratch globals, copies three words from the
/// staging area to the live area, runs the mode worker (callee 6) and the
/// weight worker (callee 7), then takes the shared exit path. When `arg0`
/// is zero and `arg1` (low byte) is non-zero it skips straight to the
/// dispatch join. When both are zero it first recomputes the time quantum
/// from the 64-bit clock (callee 10, chop-mode float conversion and a
/// fixed-point divide by 1000), picks the active quantum from the mode
/// globals, clamps it against the budget table (callee 13), accumulates it,
/// walks one of three delay paths (a poll loop over callee 14 with at most
/// a few iterations, a direct quantum path, or a second poll loop), and only
/// then reaches the dispatch join. Every path ends in the shared tail, so
/// the function always returns; the apparent loop back to the dispatch join
/// executes exactly once per call.
///
/// Globals read: the mode word, three flag bytes, the entity pointer (null
/// means "no entity"), the saved 64-bit clock, the staging words, the delay
/// bounds, the budget words and four read-only float constants. Globals
/// written: the pending count, the quantum, the accumulators, the delay
/// result slot, the scratch mirrors, the live words and the flag byte the
/// exit path clears. Callee answers used as pointers are the entity helper
/// (callee 5, words at +0 and +0x14), the budget lookup (callee 13, word at
/// +0x34) and the poll worker (callee 14, an 8-word block of which words 3
/// and 7 steer the loops).
///
/// Edge cases: a null entity pointer skips the mirror block; a zero divisor
/// in the final ratio yields infinity or NaN exactly as the scalar divide
/// instruction does; the chop-mode conversions truncate toward zero and
/// yield the indefinite low word for NaN and out-of-range inputs; the dead
/// comparison of two unequal constants in the copy block always falls
/// through to the copy; one frame store (the saved clock word) is never
/// read back and is omitted.
///
/// Original: 0x00956910 (cdecl, two stack words of which only the low bytes
/// are read; returns the last worker's result word).
lf_checker_rt::export!(cdecl, rw_00956910(arg0: u32, arg1: u32) -> u32 {
    unsafe {
        // Callee ids (see contract).
        const C_TIME_COUNT: u32 = 1;
        const C_DISPATCH: u32 = 2;
        const C_TASK_RUN: u32 = 3;
        const C_TIME_KEEP: u32 = 4;
        const C_ENTITY: u32 = 5;
        const C_MODE_WORK: u32 = 6;
        const C_WEIGHT: u32 = 7;
        const C_GATE: u32 = 8;
        const C_FLAG_CLEAR: u32 = 9;
        const C_CLOCK64: u32 = 10;
        const C_READY_A: u32 = 11;
        const C_READY_B: u32 = 12;
        const C_BUDGET: u32 = 13;
        const C_POLL: u32 = 14;
        const C_DRAIN: u32 = 15;
        const C_LIMIT_A: u32 = 16;
        const C_LIMIT_B: u32 = 17;
        const C_MODE_GATE: u32 = 18;

        // File VAs of globals.
        const G_PENDING: u32 = 0x11f7030;
        const G_DISPATCH_X: u32 = 0x11f7024;
        const G_MIR0: u32 = 0x139c234;
        const G_MIR1: u32 = 0x139c238;
        const G_ENT_MIR: u32 = 0x169e790;
        const G_ENT_FLAG: u32 = 0x169e780;
        const G_ENT_W: u32 = 0x1048178;
        const G_ENTITY: u32 = 0x118d800;
        const G_LIVE0: u32 = 0x1037810;
        const G_LIVE1: u32 = 0x1037814;
        const G_LIVEB: u32 = 0x1037818;
        const G_STAGE0: u32 = 0x11f9ffc;
        const G_STAGE1: u32 = 0x11fa000;
        const G_STAGEB: u32 = 0x11fa004;
        const G_EXIT_FLAG: u32 = 0x11f70ed;
        const G_MODE: u32 = 0x1037720;
        const G_MODE_SUB: u32 = 0x11f701e;
        const G_SAVED_LO: u32 = 0x120f2b0;
        const G_SAVED_HI: u32 = 0x120f2b4;
        const G_QUANTUM: u32 = 0x11f704c;
        const G_ACC_A: u32 = 0x11f7048;
        const G_ACC_B: u32 = 0x11f705c;
        const G_DELAY: u32 = 0x11f7054;
        const G_Q_D4: u32 = 0x11f70d4;
        const G_Q_CC: u32 = 0x11f70cc;
        const G_Q_DC: u32 = 0x11f70dc;
        const G_Q_D0: u32 = 0x11f70d0;
        const G_Q_D8: u32 = 0x11f70d8;
        const G_Q_C8: u32 = 0x11f70c8;
        const G_Q_FLAG: u32 = 0x10376e8;
        const G_BUDGET_ARG: u32 = 0x11f6f34;
        const G_BUDGET_THIS: u32 = 0x11f6954;
        const G_DELAY_LO: u32 = 0x11f7028;
        const G_DELAY_HI: u32 = 0x11f702c;
        const G_SNAP_QW: u32 = 0x120ca30;
        const G_RATIO_C: u32 = 0x120ca3c;
        const G_RATIO_D: u32 = 0x120ca5c;
        // Read-only constants.
        const C_TIME_SCALE: u32 = 0x17acd0c;
        const C_POLY_MUL: u32 = 0xe8ae9c;
        const C_RATIO_MAX: u32 = 0xfe88e8;

        const FIXED_DIV_MAGIC: u64 = 0x10624dd3;
        const FIXED_DIV_SHIFT: u32 = 6;
        const FIXED_DIV_BIAS: u32 = 0x1f4;
        const STASH_INIT: u32 = 0x21;
        const GATE_THIS: u32 = 0x128e310;
        const ONE_BITS: u32 = 0x3f800000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(a) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(a) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(a) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (lf_checker_rt::global::<u8>(a) as *mut u8).write(v) }
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
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Emulate `fistp qword` under chop (round-toward-zero) control and
        /// return the low dword, as the original's `mov reg,[esp+0x18]` does.
        /// NaN, infinity and out-of-i64-range inputs store the indefinite
        /// value whose low dword is zero.
        #[inline(always)]
        fn trunc_f32(x: f32) -> f32 {
            let b = x.to_bits();
            let e = ((b >> 23) & 0xff) as i32;
            if e < 127 {
                return f32::from_bits(b & 0x8000_0000);
            }
            if e >= 150 {
                return x;
            }
            let frac_bits = (150 - e) as u32;
            f32::from_bits(b & !((1u32 << frac_bits) - 1))
        }
        #[inline(always)]
        fn fistp_chop_low(x: f32) -> u32 {
            if x.is_nan() {
                return 0;
            }
            let t = trunc_f32(x);
            const TWO63: f32 = 9223372036854775808.0;
            if t >= TWO63 || t <= -TWO63 {
                return 0;
            }
            (t as i64) as u32
        }
        /// The original's fixed-point step: `(x * MAGIC) >> 38`.
        #[inline(always)]
        fn fixed_div(x: u32) -> u32 {
            (((FIXED_DIV_MAGIC.wrapping_mul(x as u64)) >> 32) as u32) >> FIXED_DIV_SHIFT
        }

        // Shared tail: copy block, workers, exit checks. Returns the exit word.
        unsafe fn tail() -> u32 {
            unsafe {
                // Dead branch in the original (`(an instruction of the original)` of two
                // unequal constants): the copy always runs.
                wr32(G_LIVE0, rd32(G_STAGE0));
                wr32(G_LIVE1, rd32(G_STAGE1));
                wr8(G_LIVEB, rd8(G_STAGEB));
                let _: u32 = lf_checker_rt::callee_cdecl!(C_MODE_WORK, u32,);
                let t: u32 = lf_checker_rt::callee_cdecl!(C_TIME_COUNT, u32,);
                let r: u32 = lf_checker_rt::callee_cdecl!(
                    C_WEIGHT, u32,
                    lf_checker_rt::relocated(G_STAGE0), ONE_BITS, t, 0xffff_ffff
                );
                if rd8(G_EXIT_FLAG) == 0 {
                    return r;
                }
                let x: u32 = lf_checker_rt::callee_thiscall!(C_GATE, u32, lf_checker_rt::relocated(GATE_THIS));
                if (x & 0xff) != 0 {
                    return x;
                }
                let mode = rd32(G_MODE); // also the exit word below: the original
                // returns with eax holding this load, not the gate answer.
                if (mode == 2 || mode == 7) && rd8(G_MODE_SUB) != 0 {
                    return mode;
                }
                let y: u32 = lf_checker_rt::callee_cdecl!(C_FLAG_CLEAR, u32, 0);
                wr8(G_EXIT_FLAG, 0);
                y
            }
        }

        // Dispatch join: run the task dispatcher once, then the tail.
        // `xmm1` is the float word handed to the dispatcher.
        unsafe fn join(xmm1: f32, a0: u8) -> u32 {
            unsafe {
                // The original also saves one clock word to a dead frame
                // slot here; it is never read back, so it is omitted.
                wrf(G_DISPATCH_X, xmm1);
                let qlo = rd32(G_SNAP_QW);
                let qhi = rd32(G_SNAP_QW + 4);
                let mut buf = [qlo, qhi];
                let _: u32 = lf_checker_rt::callee_cdecl!(
                    C_TASK_RUN, u32,
                    buf.as_mut_ptr() as u32, xmm1.to_bits(), 0
                );
                let _: u32 = lf_checker_rt::callee_cdecl!(C_TIME_KEEP, u32,);
                if a0 != 0 {
                    let edi = rd32(G_ENTITY);
                    if edi != 0 {
                        let v0: u32 = lf_checker_rt::callee_thiscall!(
                            C_ENTITY, u32, edi.wrapping_add(0x10)
                        );
                        let f0 = f32::from_bits(
                            (v0 as *const u32).read_unaligned()
                        );
                        let v1: u32 = lf_checker_rt::callee_thiscall!(
                            C_ENTITY, u32, edi.wrapping_add(0x10)
                        );
                        let f1 = f32::from_bits(
                            ((v1.wrapping_add(0x14)) as *const u32).read_unaligned()
                        );
                        wrf(G_MIR0, f0);
                        wrf(G_MIR1, f1);
                        wrf(G_ENT_MIR, f32::from_bits(((edi.wrapping_add(0x80)) as *const u32).read_unaligned()));
                        wrf(G_ENT_MIR + 4, f32::from_bits(((edi.wrapping_add(0x84)) as *const u32).read_unaligned()));
                        wrf(G_ENT_MIR + 8, f32::from_bits(((edi.wrapping_add(0x88)) as *const u32).read_unaligned()));
                        wrf(G_ENT_MIR + 12, f32::from_bits(((edi.wrapping_add(0x8c)) as *const u32).read_unaligned()));
                        wrf(G_ENT_W, f32::from_bits(((edi.wrapping_add(0x2c8)) as *const u32).read_unaligned()));
                    }
                    wr8(G_ENT_FLAG, 0);
                }
                tail()
            }
        }

        let a0 = (arg0 & 0xff) as u8;
        let a1 = (arg1 & 0xff) as u8;
        if a0 != 0 {
            wr32(G_PENDING, 0);
            let t: u32 = lf_checker_rt::callee_cdecl!(C_TIME_COUNT, u32,);
            let _: u32 = lf_checker_rt::callee_cdecl!(C_DISPATCH, u32, t, 1);
            return join(0.0, a0);
        }
        if a1 != 0 {
            return join(0.0, a0);
        }

        // Full recompute path (both argument bytes zero).
        let t1: u64 = lf_checker_rt::callee_cdecl!(C_CLOCK64, u64,);
        let saved = (rd32(G_SAVED_LO) as u64) | ((rd32(G_SAVED_HI) as u64) << 32);
        let delta = t1.wrapping_sub(saved);
        let f = mul((delta as i64) as f32, rdf(C_TIME_SCALE));
        let lo = fistp_chop_low(f);
        wr32(G_QUANTUM, fixed_div(lo.wrapping_add(FIXED_DIV_BIAS)));
        let t2: u64 = lf_checker_rt::callee_cdecl!(C_CLOCK64, u64,);
        wr32(G_SAVED_LO, t2 as u32);
        wr32(G_SAVED_HI, (t2 >> 32) as u32);

        // Quantum selection.
        let mut stash = STASH_INIT;
        let mut edi: u32;
        if rd32(G_Q_D4) != 0 {
            edi = rd32(G_Q_D0);
        } else if rd32(G_Q_CC) != 0 {
            let a: u32 = lf_checker_rt::callee_cdecl!(C_READY_A, u32,);
            if (a & 0xff) != 0 {
                let b = rd8(G_Q_FLAG);
                stash = if b == 0 { 0 } else { STASH_INIT };
                edi = if b == 0 { 0 } else { rd32(G_Q_C8) };
            } else {
                let e = rd32(G_Q_C8);
                let p = mul((e as f32), (lo as f32));
                let p2 = mul(p, rdf(C_POLY_MUL));
                let lo2 = fistp_chop_low(p2);
                edi = fixed_div(lo2.wrapping_add(FIXED_DIV_BIAS));
            }
        } else if rd32(G_Q_DC) != 0 {
            edi = rd32(G_Q_D8);
        } else {
            let a: u32 = lf_checker_rt::callee_cdecl!(C_READY_A, u32,);
            if (a & 0xff) != 0 {
                stash = if rd8(G_Q_FLAG) == 0 { 0 } else { stash };
                edi = stash;
            } else {
                let a2: u32 = lf_checker_rt::callee_cdecl!(C_READY_B, u32,);
                edi = rd32(G_QUANTUM);
                if (a2 & 0xff) != 0 {
                    edi = 0;
                }
            }
        }

        // Mode adjustment and accumulation.
        let mode = rd32(G_MODE);
        let sub = rd8(G_MODE_SUB);
        wr32(G_QUANTUM, edi);
        if (mode == 2 || mode == 7) && sub != 0 {
            if mode != 7 {
                edi = 0;
            }
            wr32(G_QUANTUM, edi);
        }
        let mut ebx_t0: u32;
        if (mode == 2 || mode == 7) && sub != 0 {
            ebx_t0 = rd32(G_PENDING);
        } else if rd32(G_Q_DC) != 0 {
            ebx_t0 = rd32(G_PENDING);
            let e: u32 = stash;
            wr32(G_ACC_A, rd32(G_ACC_A).wrapping_add(e));
            wr32(G_ACC_B, rd32(G_ACC_B).wrapping_add(e));
        } else {
            let barg = rd32(G_BUDGET_ARG);
            let bthis = rd32(G_BUDGET_THIS);
            let p: u32 = lf_checker_rt::callee_thiscall!(C_BUDGET, u32, bthis, barg);
            edi = rd32(G_QUANTUM);
            ebx_t0 = rd32(G_PENDING);
            let sum = edi.wrapping_add(ebx_t0);
            let cap = ((p.wrapping_add(0x34)) as *const u32).read_unaligned();
            if sum > cap {
                let p2: u32 = lf_checker_rt::callee_thiscall!(C_BUDGET, u32, bthis, barg);
                let cap2 = ((p2.wrapping_add(0x34)) as *const u32).read_unaligned();
                edi = cap2.wrapping_sub(ebx_t0);
                wr32(G_QUANTUM, edi);
                let a: u32 = lf_checker_rt::callee_cdecl!(C_READY_A, u32,);
                if !((a & 0xff) != 0 && rd8(G_Q_FLAG) == 0) {
                    let m = if edi > STASH_INIT { STASH_INIT } else { edi };
                    wr32(G_ACC_A, rd32(G_ACC_A).wrapping_add(m));
                    wr32(G_ACC_B, rd32(G_ACC_B).wrapping_add(m));
                }
                // The other arm adds zero to both accumulators: no-op.
            } else {
                let e: u32 = stash;
                wr32(G_ACC_A, rd32(G_ACC_A).wrapping_add(e));
                wr32(G_ACC_B, rd32(G_ACC_B).wrapping_add(e));
            }
        }

        // Delay paths.
        let mut xmm0: f32;
        if rd32(G_Q_D4) != 0 || rd32(G_Q_CC) != 0 {
            // Poll loop over callee 14 with a fixed-point exit value.
            let t: u32 = lf_checker_rt::callee_cdecl!(C_TIME_COUNT, u32,);
            let f_edi = edi as f32;
            let v = t.wrapping_add(fistp_chop_low(f_edi));
            let mut p: u32 =
                lf_checker_rt::callee_cdecl!(C_POLL, u32, t.wrapping_add(edi));
            let mut p3 = ((p.wrapping_add(12)) as *const u32).read_unaligned();
            if p3 > v {
                loop {
                    p = lf_checker_rt::callee_cdecl!(C_POLL, u32, p3.wrapping_sub(1));
                    p3 = ((p.wrapping_add(12)) as *const u32).read_unaligned();
                    if p3 <= v {
                        break;
                    }
                }
            }
            xmm0 = f_edi;
            let lim = rd32(G_DELAY_HI).wrapping_sub(rd32(G_DELAY_LO));
            let sum = edi.wrapping_add(ebx_t0);
            if sum > lim {
                wr32(G_PENDING, lim);
                xmm0 = 0.0;
            } else {
                wr32(G_PENDING, sum);
            }
        } else if rd32(G_Q_DC) == 0 {
            // Poll loop over callee 14 with a float-scaled exit value.
            let t: u32 = lf_checker_rt::callee_cdecl!(C_TIME_COUNT, u32,);
            let f_edi = edi as f32;
            let mut p: u32 =
                lf_checker_rt::callee_cdecl!(C_POLL, u32, t.wrapping_add(edi));
            loop {
                let p3 = ((p.wrapping_add(12)) as *const u32).read_unaligned();
                let p7 = f32::from_bits(
                    ((p.wrapping_add(28)) as *const u32).read_unaligned()
                );
                let v = t.wrapping_add(fistp_chop_low(mul(f_edi, p7)));
                if p3 <= v {
                    break;
                }
                p = lf_checker_rt::callee_cdecl!(C_POLL, u32, p3.wrapping_sub(1));
            }
            xmm0 = f_edi;
            ebx_t0 = ebx_t0.wrapping_add(edi);
            wr32(G_PENDING, ebx_t0);
        } else if edi > ebx_t0 {
            xmm0 = ebx_t0 as f32;
            wr32(G_PENDING, 0);
        } else {
            xmm0 = edi as f32;
            wr32(G_PENDING, ebx_t0.wrapping_sub(edi));
        }
        wrf(G_DELAY, xmm0);

        // Limit checks, then the ratio gate.
        let t: u32 = lf_checker_rt::callee_cdecl!(C_TIME_COUNT, u32,);
        let la: u32 = lf_checker_rt::callee_cdecl!(C_LIMIT_B, u32, t);
        if (la & 0xff) == 0 {
            let t2: u32 = lf_checker_rt::callee_cdecl!(C_TIME_COUNT, u32,);
            let lb: u32 = lf_checker_rt::callee_cdecl!(C_LIMIT_A, u32, t2);
            if (lb & 0xff) == 0 {
                let lc: u32 = lf_checker_rt::callee_cdecl!(C_DISPATCH, u32, t2, 0);
                if (lc & 0xff) == 0 {
                    let t3: u32 = lf_checker_rt::callee_cdecl!(C_TIME_COUNT, u32,);
                    let dd = rd32(G_RATIO_D);
                    if dd < t3 {
                        return join(rdf(C_RATIO_MAX), a0);
                    }
                    let cc = rd32(G_RATIO_C);
                    let f2 = div(
                        (t3.wrapping_sub(cc)) as f32,
                        (dd.wrapping_sub(cc)) as f32,
                    );
                    let c0 = rdf(C_RATIO_MAX);
                    if f2 > c0 {
                        return join(c0, a0);
                    }
                    if 0.0f32 > f2 {
                        return join(0.0, a0);
                    }
                    return join(f2, a0);
                }
            }
        }
        let mg: u32 = lf_checker_rt::callee_cdecl!(C_MODE_GATE, u32,);
        if (mg & 0xff) != 0 {
            return join(0.0, a0);
        }
        let t4: u32 = lf_checker_rt::callee_cdecl!(C_TIME_COUNT, u32,);
        let ld: u32 = lf_checker_rt::callee_cdecl!(C_LIMIT_A, u32, t4);
        if (ld & 0xff) != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(C_DRAIN, u32,);
            return join(0.0, a0);
        }
        let le: u32 = lf_checker_rt::callee_cdecl!(C_LIMIT_B, u32, t4);
        if (le & 0xff) == 0 {
            return join(0.0, a0);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(C_DRAIN, u32,);
        join(rdf(C_RATIO_MAX), a0)
    }
});

