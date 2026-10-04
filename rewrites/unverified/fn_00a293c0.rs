// original: 0x00a293c0 task_aim_assist_init (proposed)

/// Initialise a ped task's aim-assist state from a target, a scale and a
/// parameter block, through one of three assist paths.
///
/// `this` is the task (thiscall, three stack words: `a1` a state block,
/// `a2` a float scale as bits, `a3` a parameter block). The prologue stores
/// `a3+PARAM_OFF` into `+HANDLE_OFF`, fixes several counters and flags (byte
/// 1 at `+READY_OFF`, `0xEA60` at `+TIME_OFF`, zeros, 1.0f at `+RATE_OFF`)
/// and, when bit 1 of the flag byte at `+FLAG_OFF` is set, seeds `+ASSIST_OFF`
/// from `a3+A3_SEED_OFF`. Bit 1 is kept as `cl` below and bit 2 as `flag2`.
///
/// The target path runs when `a1+T Bit4` has bit 2 set and `a1+LINK_OFF` is
/// non-null: with `cl`, `+AUX_OFF` takes `a1+FLOAT_OFF` minus the scale;
/// then the id 1 helper (xmm0 in and out; the rewrite passes the value as a
/// stack word and the stub transports it, comparing only XMM0) runs on the
/// float at `[[LINK]+0x20]+0x18` clamped into `[-1, 1]`, and the answer is
/// added into `+ASSIST_OFF`.
///
/// The search path runs when `a1+ALT_OFF` is null. With `cl`, the id 2
/// search callee runs (thiscall on `this`: two scratch words, `&this+AUX`,
/// `a1`); the scratch addresses are skipped in the comparison (their
/// contents are uninitialised on the original side), while the callee's
/// three one-word
/// out-params (verified from its body) are scripted. A zero answer with both
/// global bytes clear stores `FLOAT+scale` into `+AUX_OFF` when `flag2` is
/// set (otherwise it skips ahead); a set global byte stores the alternate
/// global instead. Clear global bytes then run the dot-product block when
/// `a1+T Bit0` has bit 0: the triple at `a1+DOT_OFF` dotted with
/// `a1[VEC]+0x10` in `(b1+b0)+b2` order, clamped into `[-1, 1]`, answered by
/// id 1 again and subtracted from `+ASSIST_OFF`. Set global bytes run the
/// lookup block instead: the id 3 and id 4 lookup callees (same target, two
/// sites, scripted pointer-or-null each) feed, when non-null and `flag2`,
/// one float each (`+0x304`, `+0x190`) into `+ASSIST_OFF`, re-zeroed when the
/// value is strictly between the reference (exclusive) and zero; two nulls
/// with `flag2` store the id 5 filtered global there.
///
/// The direct path runs otherwise: two triples (from `a1+ALT_OFF`, either
/// `+0x10` or through `+0x20` plus `0x30`, and from `a1+VEC_OFF+0x30`) are
/// differenced, and unless the sum of squares is ordered-equal to zero the
/// differences are normalised by its square root; the id 6 combine callee
/// (cdecl, two scratch words and `&this+AUX`, one scripted out-param word
/// verified from its body) then runs, and the three scaled differences plus
/// the out-param word are stored as four words to `VEC_GLOBAL` (the fourth
/// word re-reads the caller's own scratch, which the contract defines to
/// zero, as does the rewrite).
///
/// The tail always runs: the id 7 rating callee (thiscall, `a1` and 1; its
/// float answer, also written to `a1+0x1f4` by the real callee, is scripted
/// once for both) into `+SCORE_OFF`, then the id 8 pose callee (thiscall on
/// `this+0x10` with the assist and aux words; its eleven output words at
/// `+0x10` verified from its nested callee's body are scripted), and bit 2
/// of the flag byte is set. Ordered compares repeat the original exactly.
/// There is no designed return value (EAX ends as callee residue), so the
/// contract compares everything except it.
///
/// Original: 0x00a293c0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00a293c0(this: u32, a1: u32, a2bits: u32, a3: u32) -> u32 {
    unsafe {
        const PARAM_OFF: u32 = 0x0018;
        const HANDLE_OFF: u32 = 0x01f0;
        const READY_OFF: u32 = 0x023c;
        const TIME_OFF: u32 = 0x0238;
        const TIME_VAL: u32 = 0xea60;
        const ZERO_A: u32 = 0x0240;
        const ZERO_B: u32 = 0x0220;
        const ZERO_C: u32 = 0x0224;
        const RATE_OFF: u32 = 0x0228;
        const ONE_BITS: u32 = 0x3f80_0000;
        const ZERO_D: u32 = 0x0178;
        const ZERO_E: u32 = 0x0174;
        const ZERO_F: u32 = 0x0170;
        const FLAG_OFF: u32 = 0x0216;
        const ASSIST_OFF: u32 = 0x0218;
        const AUX_OFF: u32 = 0x021c;
        const A3_SEED_OFF: u32 = 0x000c;
        const TSTATE_OFF: u32 = 0x026c;
        const LINK_OFF: u32 = 0x0b30;
        const FLOAT_OFF: u32 = 0x0aa0;
        const ALT_OFF: u32 = 0x0398;
        const VEC_OFF: u32 = 0x0020;
        const DOT_OFF: u32 = 0x0b00;
        const SCORE_OFF: u32 = 0x01f4;
        const GLOB_A: u32 = 0x0103_ce46;
        const GLOB_B: u32 = 0x0103_ce47;
        const ALT_GLOBAL: u32 = 0x0128_e3a0;
        const FILT_GLOBAL: u32 = 0x0128_e328;
        const VEC_GLOBAL: u32 = 0x012d_d600;
        const REF_C: u32 = 0x00fe_8d68;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        /// Clamp into [-1, 1] with the original's ordered-compare shape
        /// (both clamp sites map to this; NaN passes through).
        #[inline(always)]
        fn clamp11(v: f32) -> f32 {
            if v > 1.0 {
                1.0
            } else if -1.0 > v {
                -1.0
            } else {
                v
            }
        }

        wr32(this.wrapping_add(HANDLE_OFF), rd32(a3.wrapping_add(PARAM_OFF)));
        wr8(this.wrapping_add(READY_OFF), 1);
        wr32(this.wrapping_add(TIME_OFF), TIME_VAL);
        wr32(this.wrapping_add(ZERO_A), 0);
        wr32(this.wrapping_add(ZERO_B), 0);
        wr32(this.wrapping_add(ZERO_C), 0);
        wr32(this.wrapping_add(RATE_OFF), ONE_BITS);
        wr32(this.wrapping_add(ZERO_D), 0);
        wr32(this.wrapping_add(ZERO_E), 0);
        wr32(this.wrapping_add(ZERO_F), 0);
        let flags = rd8(this.wrapping_add(FLAG_OFF));
        let cl = (flags >> 1) & 1 != 0;
        let flag2 = flags & 2 != 0;
        if cl {
            wr32(this.wrapping_add(ASSIST_OFF), rd32(a3.wrapping_add(A3_SEED_OFF)));
        }
        let a2 = f32::from_bits(a2bits);
        let ga = rd8(lf_checker_rt::relocated(GLOB_A));
        let gb = rd8(lf_checker_rt::relocated(GLOB_B));
        if rd8(a1.wrapping_add(TSTATE_OFF)) & 4 != 0 && rd32(a1.wrapping_add(LINK_OFF)) != 0 {
            if cl {
                let f = rdf(a1.wrapping_add(FLOAT_OFF));
                wrf(this.wrapping_add(AUX_OFF), f - a2);
            }
            let l0 = rd32(a1.wrapping_add(LINK_OFF));
            let l1 = rd32(l0.wrapping_add(0x20));
            let v = clamp11(rdf(l1.wrapping_add(0x18)));
            let ans: u32 = lf_checker_rt::callee_cdecl!(1, u32, v.to_bits());
            let old = rdf(this.wrapping_add(ASSIST_OFF));
            wrf(this.wrapping_add(ASSIST_OFF), f32::from_bits(ans) + old);
        } else if rd32(a1.wrapping_add(ALT_OFF)) == 0 {
            if cl {
                let mut s0 = [0u32; 1];
                let mut s1 = [0u32; 1];
                // Argument order is the reverse of the original's pushes:
                // the last push (a1) is argument 0.
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    2,
                    u32,
                    this,
                    a1,
                    this.wrapping_add(AUX_OFF),
                    s1.as_mut_ptr() as u32,
                    s0.as_mut_ptr() as u32
                );
                if (r as u8) == 0 {
                    if ga == 0 && gb == 0 {
                        if flag2 {
                            let f = rdf(a1.wrapping_add(FLOAT_OFF));
                            wrf(this.wrapping_add(AUX_OFF), f + a2);
                        }
                    } else if flag2 {
                        wrf(this.wrapping_add(AUX_OFF), rdf(lf_checker_rt::relocated(ALT_GLOBAL)));
                    }
                }
            }
            if ga != 0 || gb != 0 {
                let p1: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, 1, 0, 0);
                if p1 != 0 {
                    if flag2 {
                        let x = rdf(p1.wrapping_add(0x304));
                        wrf(this.wrapping_add(ASSIST_OFF), x);
                        let c = rdf(lf_checker_rt::relocated(REF_C));
                        if 0.0 > x && x > c {
                            wr32(this.wrapping_add(ASSIST_OFF), 0);
                        }
                    }
                } else {
                    let p2: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, 2, 0, 0);
                    if p2 != 0 {
                        if flag2 {
                            let x = rdf(p2.wrapping_add(0x190));
                            wrf(this.wrapping_add(ASSIST_OFF), x);
                            let c = rdf(lf_checker_rt::relocated(REF_C));
                            if 0.0 > x && x > c {
                                wr32(this.wrapping_add(ASSIST_OFF), 0);
                            }
                        }
                    } else if flag2 {
                        let d: f32 = lf_checker_rt::callee_cdecl!(
                            5,
                            f32,
                            rd32(lf_checker_rt::relocated(FILT_GLOBAL))
                        );
                        wrf(this.wrapping_add(ASSIST_OFF), d);
                    }
                }
            } else {
                // Both global bytes clear (the no-store sub-path joins here
                // directly, with the same bytes).
                if rd8(a1.wrapping_add(TSTATE_OFF)) & 1 != 0 {
                    let v = rd32(a1.wrapping_add(VEC_OFF));
                    let d0 = rdf(a1.wrapping_add(DOT_OFF)) * rdf(v.wrapping_add(0x10));
                    let d1 = rdf(a1.wrapping_add(DOT_OFF + 4)) * rdf(v.wrapping_add(0x14));
                    let mut dot = d1 + d0;
                    let d2 = rdf(a1.wrapping_add(DOT_OFF + 8)) * rdf(v.wrapping_add(0x18));
                    dot = dot + d2;
                    let ans: u32 =
                        lf_checker_rt::callee_cdecl!(1, u32, clamp11(dot).to_bits());
                    let old = rdf(this.wrapping_add(ASSIST_OFF));
                    wrf(this.wrapping_add(ASSIST_OFF), old - f32::from_bits(ans));
                }
            }
        } else {
            let alt = rd32(a1.wrapping_add(ALT_OFF));
            let link = rd32(alt.wrapping_add(0x20));
            let base = if link != 0 { link.wrapping_add(0x30) } else { alt.wrapping_add(0x10) };
            let v = rd32(a1.wrapping_add(VEC_OFF));
            let x5 = rdf(base.wrapping_add(4)) - rdf(v.wrapping_add(0x34));
            let x4 = rdf(base) - rdf(v.wrapping_add(0x30));
            let x6 = rdf(base.wrapping_add(8)) - rdf(v.wrapping_add(0x38));
            let mut n = x4 * x4;
            n = n + x5 * x5;
            n = n + x6 * x6;
            // The original skips unless the sum is ordered-unequal to zero
            // (ucomiss/lahf/test/jnp); `!=` matches, NaN included.
            let mut inv = 0.0f32;
            if n != 0.0 {
                inv = 1.0 / n.sqrt();
            }
            let m4 = x4 * inv;
            let m0 = inv * x5;
            let m2 = inv * x6;
            // The original passes the m4 slot to id 6, whose stub overwrites
            // it, then stores four words starting there: out-param, m0, m2,
            // and the next scratch word, which nothing ever wrote (zero under
            // the contract's stack fill on both sides).
            let mut slot = m4;
            let mut dummy = [0u32; 1];
            let _: u32 = lf_checker_rt::callee_cdecl!(
                6,
                u32,
                (&mut slot as *mut f32) as u32,
                dummy.as_mut_ptr() as u32,
                this.wrapping_add(AUX_OFF)
            );
            let g = lf_checker_rt::relocated(VEC_GLOBAL);
            wrf(g, slot);
            wrf(g.wrapping_add(4), m0);
            wrf(g.wrapping_add(8), m2);
            wr32(g.wrapping_add(12), 0);
        }
        let f: f32 = lf_checker_rt::callee_thiscall!(7, f32, this, a1, 1);
        wrf(this.wrapping_add(SCORE_OFF), f);
        let w0 = rd32(this.wrapping_add(ASSIST_OFF));
        let w1 = rd32(this.wrapping_add(AUX_OFF));
        let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, this.wrapping_add(0x10), w0, w1);
        wr8(this.wrapping_add(FLAG_OFF), rd8(this.wrapping_add(FLAG_OFF)) | 2);
        0
    }
});
