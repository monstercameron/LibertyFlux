// original: 0x00da94a0 task_pick_best_candidate (proposed)

/// Scan a few table-driven candidate positions and keep the best one.
///
/// `pos` points to three floats (the query point). `frame` points to a
/// 3x3 row matrix (rows at `+0x00`, `+0x10`, `+0x20`) plus a translation
/// triple at `+0x30`. `obj_a` points to an object whose word at `+0x20`
/// points to a second triple. `ctx` points to a context holding an array
/// base at `+0x8c`, six blend floats at `+0x90`..`+0xa8` and a word-table
/// base at `+0xb0`. `index` selects one 32-byte record in the array; the
/// record's words at `+0x10` pick table rows and its word at `+0x16`
/// selects 3 or 4 scan steps. `out_a`/`out_b` receive two marker words
/// and `best` points to the running-best value, updated in place.
///
/// The residual `pos - translation` is rotated by the frame rows into
/// `r`, and the second triple is rotated by the same rows into `s`; each
/// row dot product is ordered `(m1*d1 + m0*d0) + m2*d2`. Each scan step
/// builds a candidate triple from the word table (`base + scale * word`),
/// calls the projection callee with the previous triple, the current
/// triple, `r` and an output slot, then forms the squared length of the
/// output-minus-`r` residual. The step whose residual length plus a
/// read-only constant beats `*best` (with a zero-length guard through the
/// length callee and a sign test on a weighted sum) stores its residual
/// length into `*best` and its markers into `*out_a`/`*out_b`, swapped by
/// the sign. Returns 1 when any step improved the best, else 0.
///
/// Two original behaviours are deliberately not reproduced: the compiler
/// spills one pointer into its own incoming stack slot (unobservable
/// except to the checker's stack check, which is off for this contract),
/// and one dead frame slot is read and stored into another dead slot
/// (both below the stack-compare window, value 0 under the defined fill).
///
/// Original: 0x00da94a0 (cdecl, eight stack words, byte result in `al`).
lf_checker_rt::export!(cdecl, rw_00DA94A0(obj_a: u32, pos: u32, ctx: u32, index: u32, frame: u32, out_a: u32, out_b: u32, best: u32) -> u32 {
    unsafe {
        const ROW0: u32 = 0x00;
        const ROW1: u32 = 0x10;
        const ROW2: u32 = 0x20;
        const TR0: u32 = 0x30;
        const TR1: u32 = 0x34;
        const TR2: u32 = 0x38;
        const OBJA_VEC: u32 = 0x20;
        const CTX_ARR: u32 = 0x8c;
        const CTX_C0: u32 = 0x90;
        const CTX_C1: u32 = 0x94;
        const CTX_C2: u32 = 0x98;
        const CTX_C3: u32 = 0xa0;
        const CTX_C4: u32 = 0xa4;
        const CTX_C5: u32 = 0xa8;
        const CTX_TAB: u32 = 0xb0;
        const REC_STRIDE: u32 = 5;
        const REC_WORD: u32 = 0x10;
        const REC_FLAG: u32 = 0x16;
        const STEP_BASE: u32 = 3;
        const CALLEE_PROJECT: u32 = 0;
        const CALLEE_LEN: u32 = 1;
        const K1_VA: u32 = 0x00fe865c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        /// A table word as the original's `movsx` + `cvtdq2ps` forms it.
        #[inline(always)]
        unsafe fn wtof(a: u32) -> f32 {
            unsafe { (((a as *const u16).read_unaligned() as i16) as i32) as f32 }
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

        // Residual of the query point against the frame translation.
        let d0 = sub(rdf(pos), rdf(frame + TR0));
        let d1 = sub(rdf(pos + 4), rdf(frame + TR1));
        let d2 = sub(rdf(pos + 8), rdf(frame + TR2));
        let r00 = rdf(frame + ROW0);
        let r01 = rdf(frame + ROW0 + 4);
        let r02 = rdf(frame + ROW0 + 8);
        let r10 = rdf(frame + ROW1);
        let r11 = rdf(frame + ROW1 + 4);
        let r12 = rdf(frame + ROW1 + 8);
        let r20 = rdf(frame + ROW2);
        let r21 = rdf(frame + ROW2 + 4);
        let r22 = rdf(frame + ROW2 + 8);
        let r0 = add(add(mul(r01, d1), mul(r00, d0)), mul(r02, d2));
        let r1 = add(add(mul(r11, d1), mul(r10, d0)), mul(r12, d2));
        let r2 = add(add(mul(r21, d1), mul(r20, d0)), mul(r22, d2));
        // The second triple through the same rows.
        let q = rd32(obj_a + OBJA_VEC);
        let q0 = rdf(q);
        let q1 = rdf(q + 4);
        let q2 = rdf(q + 8);
        let s0 = add(add(mul(r00, q0), mul(r01, q1)), mul(r02, q2));
        let s1 = add(add(mul(r10, q0), mul(r11, q1)), mul(r12, q2));
        let s2 = add(add(mul(r20, q0), mul(r21, q1)), mul(r22, q2));
        // Candidate record, word table, blend constants.
        let rec = index.wrapping_shl(REC_STRIDE).wrapping_add(rd32(ctx + CTX_ARR));
        let tbl = rd32(ctx + CTX_TAB);
        let c0 = rdf(ctx + CTX_C0);
        let c1 = rdf(ctx + CTX_C1);
        let c2 = rdf(ctx + CTX_C2);
        let c3 = rdf(ctx + CTX_C3);
        let c4 = rdf(ctx + CTX_C4);
        let c5 = rdf(ctx + CTX_C5);
        let w0 = rd16(rec + REC_WORD);
        let t0 = wtof(tbl + w0.wrapping_mul(6));
        let t1 = wtof(tbl + w0.wrapping_mul(6) + 2);
        let t2 = wtof(tbl + w0.wrapping_mul(6) + 4);
        let mut prev = [add(c3, mul(c0, t0)), add(c4, mul(c1, t1)), add(c5, mul(c2, t2))];
        let count = STEP_BASE + u32::from(rd16(rec + REC_FLAG) != 0);
        let mut w_stored = w0;
        let mut found: u32 = 0xffff_ffff;
        let k1 = f32::from_bits(rd32(lf_checker_rt::relocated(K1_VA)));
        let mut esi = 1u32;
        loop {
            let rem = esi % count;
            let w = rd16(rec + rem * 2 + REC_WORD);
            let u0 = add(c3, mul(c0, wtof(tbl + w.wrapping_mul(6))));
            let u1 = add(c4, mul(c1, wtof(tbl + w.wrapping_mul(6) + 2)));
            let u2 = add(c5, mul(c2, wtof(tbl + w.wrapping_mul(6) + 4)));
            let cur = [u0, u1, u2];
            let r = [r0, r1, r2];
            let mut out = [0.0f32; 3];
            lf_checker_rt::callee_cdecl!(CALLEE_PROJECT, u32,
                &prev as *const [f32; 3] as u32,
                &cur as *const [f32; 3] as u32,
                &r as *const [f32; 3] as u32,
                &mut out as *mut [f32; 3] as u32);
            let g0 = sub(out[0], r0);
            let g1 = sub(out[1], r1);
            let g2 = sub(out[2], r2);
            let lensq_a = add(add(mul(g1, g1), mul(g0, g0)), mul(g2, g2));
            let du0 = sub(u0, prev[0]);
            let du1 = sub(u1, prev[1]);
            let du2 = sub(u2, prev[2]);
            let lensq_b = add(add(mul(du1, du1), mul(du0, du0)), mul(du2, du2));
            let root: f32 = lf_checker_rt::callee_cdecl!(CALLEE_LEN, f32, lensq_b.to_bits());
            let t = add(
                add(mul(mul(du1, root), s1), mul(mul(du0, root), s0)),
                mul(mul(du2, root), s2),
            );
            // Taken exactly when the original's `comiss` + `jbe` skips.
            if !(t.is_nan() || t == 0.0) {
                let lim = add(lensq_a, k1);
                let b = rdf(best);
                if !(b.is_nan() || lim.is_nan() || b <= lim) {
                    wrf(best, lensq_a);
                    found = esi;
                    if t <= 0.0 {
                        wr32(out_a, w);
                        wr32(out_b, w_stored);
                    } else {
                        wr32(out_a, w_stored);
                        wr32(out_b, w);
                    }
                }
            }
            prev = cur;
            w_stored = w;
            esi += 1;
            if esi > count {
                break;
            }
        }
        if (found as i32) < 0 { 0 } else { 1 }
    }
});
