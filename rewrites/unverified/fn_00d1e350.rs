// original: 0x00d1e350 CTaskComplexSeekCoverShooting::vf20
/// Run one cover-shooting think tick: poll the perception state, pick a
/// shooting sub-task and update the aim.
///
/// `this` is the task (subject at `+8`), `ped` the ped. The subject's state
/// slot (`+0xc`, id 1) routes first: 0x2ca exits with the subject. A memory
/// comparison below 30 continues, otherwise id 2 plus callees 10..12 run and
/// exit with the subject. Tag, flag bytes and three more state polls
/// (0x119/0x76d/0x41a) select the middle path; anything else falls into a
/// guard chain (callees 13..15) that either exits with id 4's answer or
/// drops into the late path.
///
/// The middle path samples callee 16 into four words (copied to
/// `this+0xa0..0xac`), fetches the pool worker (id 17) to run callee 18,
/// classifies the target (id 19, optional integer scaler id 20 and flag
/// setter id 21), subtracts the ped position, runs the aim-blend callee
/// (id 23, float answer plus out byte) and the range callee (id 24), then
/// either exits zero, exits with id 25's answer, or drops late.
///
/// The late path re-polls the state (0x11d), resolves the target (id 26),
/// checks its kind (id 5, 0x3ae/0x384) and index (id 27 against the signed
/// byte at `+0x98`, setting bits at the ped's `+0xa80` block on match),
/// \(optionally id 28 and another guard chain, then samples id 29 and runs
/// three float gates (a root distance strictly between two constants, a
/// threshold choice from id 6 (4.0/2.5) against `+0xb8`, a blended
/// difference strictly between two more constants) and a final dot sign.
/// Passing all of them runs the probe (id 32), the validator (id 33), the
/// guards again and the executor (id 34), exiting with its answer.
///
/// The tail re-polls (0x414), optionally toggles (id 8) and notifies
/// (ids 35..36), then either exits with the subject or runs the lock slot
/// (id 3) and the commit slot (id 9), exiting with the commit's answer.
/// Float operation order is the original's throughout; comparisons are
/// strict with NaN failing shut, matching the original's conditional jumps.
/// Original: 0x00d1e350 (thiscall, one stack arg).
lf_checker_rt::export!(thiscall, rw_00d1e350(this: u32, ped: u32) -> u32 {
    unsafe {
        const SUB: u32 = 0x08;
        const PED_POS: u32 = 0x20;
        const PED_EXTRA: u32 = 0x224;
        const PED_TAG: u32 = 0xd68;
        const PED_AUX: u32 = 0xa80;
        const POOL: u32 = 0x0167e2a0;
        const STATE_SLOT: u32 = 0x0c;

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
        unsafe fn or32(a: u32, v: u32) {
            unsafe { wr32(a, rd32(a) | v) }
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
        fn subf(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn icall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn icall1(obj: u32, slot: u32, a0: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0)
            }
        }
        #[inline(always)]
        unsafe fn icall3(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0, a1, a2)
            }
        }
        // Tail (0xD1EA9C on): final toggle, notify and commit.
        #[inline(always)]
        unsafe fn tail(this: u32, ped: u32) -> u32 {
            unsafe {
                let sub = rd32(this + SUB);
                if icall0(sub, STATE_SLOT) != 0x414 {
                    let t: u32 = icall0(this, 0x58);
                    if (t & 0xff) != 0 {
                        lf_checker_rt::callee_thiscall!(35, u32, ped, 1);
                        let b = rd32(this + 0xb0);
                        let c = rd32(b + 0x20);
                        let arg = if c != 0 { c.wrapping_add(0x30) } else { b.wrapping_add(0x10) };
                        lf_checker_rt::callee_thiscall!(36, u32, ped, arg);
                    }
                }
                if (rd8(this + 0x94) & 2) == 0 {
                    return rd32(this + SUB);
                }
                if (rd8(sub + STATE_SLOT) & 1) == 0 {
                    let r: u32 = icall3(sub, 0x14, ped, 1, 0);
                    if (r & 0xff) == 0 {
                        return rd32(this + SUB);
                    }
                    or32(sub + STATE_SLOT, 2);
                }
                icall1(this, 0x48, ped)
            }
        }

        // Late-float path (0xD1E8D2 on): second aim blend, gates, execute.
        #[inline(always)]
        unsafe fn late_float(this: u32, ped: u32, sq: f32, out40: [u32; 3]) -> u32 {
            unsafe {
                let pos = rd32(ped + PED_POS);
                let px = subf(rdf(this + 0x20), rdf(pos + 0x30));
                let py = subf(rdf(this + 0x24), rdf(pos + 0x34));
                let pz = subf(rdf(this + 0x28), rdf(pos + 0x38));
                let tag = rd32(ped + PED_TAG);
                let mut o60 = [0u32; 3];
                let mut o50 = [0u32; 3];
                o50[0] = px.to_bits();
                o50[1] = py.to_bits();
                o50[2] = pz.to_bits();
                lf_checker_rt::callee_thiscall!(22, u32, tag, o60.as_mut_ptr() as u32, o50.as_mut_ptr() as u32);
                let mut obyte = 0u32;
                let mut vecdum = [0u32; 3];
                let fret: u32 = lf_checker_rt::callee_cdecl!(23, u32, ped, vecdum.as_mut_ptr() as u32, &mut obyte as *mut u32 as u32);
                let h: u32 = icall0(ped, 0x128);
                let thr = f32::from_bits(rd32(lf_checker_rt::relocated(if (h & 0xff) != 0 {
                    0x00fe8ab8
                } else {
                    0x00fe8a60
                })));
                if rdf(this + 0xb8) > thr && !(thr < sq) {
                    let r: u32 = lf_checker_rt::callee_thiscall!(24, u32, this, ped, sq.to_bits(), obyte);
                    let _ = r;
                }
                let base = add(
                    f32::from_bits((lf_checker_rt::global::<u32>(0x0171de7c) as *const u32).read()),
                    rdf(this + 0xb8),
                );
                let diff = subf(sq, base);
                let c3 = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe879c)));
                if !(diff < c3) {
                    return tail(this, ped);
                }
                let c4 = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe8d7c)));
                if !(diff > c4) {
                    return tail(this, ped);
                }
                let p2 = rd32(ped + PED_POS);
                let t30 = subf(rdf(p2 + 0x30), f32::from_bits(out40[0]));
                let t34 = subf(rdf(p2 + 0x34), f32::from_bits(out40[1]));
                let t38 = subf(rdf(p2 + 0x38), f32::from_bits(out40[2]));
                let mut probe = [t30.to_bits(), t34.to_bits(), t38.to_bits()];
                lf_checker_rt::callee_thiscall!(32, u32, probe.as_mut_ptr() as u32);
                let t30 = f32::from_bits(probe[0]);
                let t34 = f32::from_bits(probe[1]);
                let t38 = f32::from_bits(probe[2]);
                let u0 = mul(rdf(p2 + 0x10), t30);
                let u1 = mul(rdf(p2 + 0x14), t34);
                let u1 = add(u1, u0);
                let u0b = mul(rdf(p2 + 0x18), t38);
                let dot = add(u1, u0b);
                if !(dot < 0.0) {
                    return tail(this, ped);
                }
                let mut o_a = [0u32; 3];
                let mut o_b = [0u32; 3];
                let vok: u32 = lf_checker_rt::callee_cdecl!(33, u32, rd32(ped + PED_TAG), ped, o_a.as_mut_ptr() as u32, o_b.as_mut_ptr() as u32, 0);
                if (vok & 0xff) == 0 {
                    return tail(this, ped);
                }
                let sub = rd32(this + SUB);
                let g: u32 = lf_checker_rt::callee_thiscall!(37, u32, sub, ped, 1, 0);
                if (g & 0xff) == 0 {
                    return tail(this, ped);
                }
                let pool = (lf_checker_rt::global::<u32>(POOL) as *const u32).read();
                let w: u32 = lf_checker_rt::callee_thiscall!(17, u32, pool);
                if w == 0 {
                    return 0;
                }
                let mut dum = [0u32; 3];
                lf_checker_rt::callee_thiscall!(34, u32, w, 0x2a, rd32(this + 0xb4), dum.as_mut_ptr() as u32, fret, 1, 0)
            }
        }

        // Late-rest path (0xD1E7C1 on): target gates, float gates, execute.
        #[inline(always)]
        unsafe fn late_rest(this: u32, ped: u32) -> u32 {
            unsafe {
                let sub = rd32(this + SUB);
                if rd32(ped + PED_TAG) == 0 {
                    return tail(this, ped);
                }
                if icall0(sub, STATE_SLOT) != 0x11d {
                    return tail(this, ped);
                }
                if rd32(this + 0xb4) == 0xffff_ffff {
                    return tail(this, ped);
                }
                let tgt: u32 = lf_checker_rt::callee_thiscall!(26, u32, sub, ped);
                if tgt == 0 {
                    return tail(this, ped);
                }
                if icall0(tgt, STATE_SLOT) != 0x3ae {
                    return tail(this, ped);
                }
                let sx = rd8(tgt + 0x98) as i8 as i32 as u32;
                let idx: u32 = lf_checker_rt::callee_thiscall!(27, u32, tgt);
                if sx != idx.wrapping_sub(1) {
                    return tail(this, ped);
                }
                let mut out70 = [0u32; 3];
                let mut out40 = [0u32; 3];
                let go: u32 = lf_checker_rt::callee_thiscall!(29, u32, tgt, ped, out70.as_mut_ptr() as u32, out40.as_mut_ptr() as u32, 0);
                if (go & 0xff) == 0 {
                    return tail(this, ped);
                }
                let pos = rd32(ped + PED_POS);
                let g1 = subf(rdf(pos + 0x30), f32::from_bits(out40[0]));
                let g0 = subf(rdf(pos + 0x34), f32::from_bits(out40[1]));
                let g1 = mul(g1, g1);
                let g0 = mul(g0, g0);
                let gsum = add(g0, g1);
                let sq = core::hint::black_box(gsum).sqrt();
                let c_lo = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe8a24)));
                if !(sq > c_lo) {
                    return late_float(this, ped, sq, out40);
                }
                let c_hi = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe8ae0)));
                if !(sq < c_hi) {
                    return late_float(this, ped, sq, out40);
                }
                let picked: u32 = icall1(this, 0x3c, 0x41a);
                if picked != 0 {
                    let pool = (lf_checker_rt::global::<u32>(POOL) as *const u32).read();
                    let w: u32 = lf_checker_rt::callee_thiscall!(17, u32, pool);
                    let arg = if w == 0 {
                        0
                    } else {
                        lf_checker_rt::callee_thiscall!(30, u32, w, 0, 0)
                    };
                    lf_checker_rt::callee_thiscall!(31, u32, sub, arg);
                }
                late_float(this, ped, sq, out40)
            }
        }

        // Late path (0xD1E6EC on): re-resolve the target, run the float
        // gates and either execute or fall into the tail. Defined before
        // the main body so every branch can reach it.
        #[inline(always)]
        unsafe fn late(this: u32, ped: u32) -> u32 {
            unsafe {
                let sub = rd32(this + SUB);
                if icall0(sub, STATE_SLOT) == 0x11d {
                    let tgt: u32 = lf_checker_rt::callee_thiscall!(26, u32, sub, ped);
                    if tgt != 0 {
                        if icall0(tgt, STATE_SLOT) == 0x3ae {
                            let sx = rd8(tgt + 0x98) as i8 as i32 as u32;
                            let idx: u32 = lf_checker_rt::callee_thiscall!(27, u32, tgt);
                            if sx == idx.wrapping_sub(1) {
                                let a = rd32(ped + PED_AUX);
                                or32(a + 0x50, 0x80);
                                let a = rd32(ped + PED_AUX);
                                or32(a + 0x50, 0x100);
                            }
                            let k: u32 = lf_checker_rt::callee_thiscall!(28, u32, tgt);
                            let armed = if k == 2 {
                                true
                            } else {
                                let tag = rd32(ped + PED_TAG);
                                tag != 0 && ((rd32(tag) >> 11) & 1) != 0
                            };
                            if armed {
                                let ok: u32 = lf_checker_rt::callee_thiscall!(14, u32, this, ped, 1, 0);
                                if (ok & 0xff) != 0 {
                                    return 0;
                                }
                            }
                            return late_rest(this, ped);
                        }
                        if icall0(tgt, STATE_SLOT) == 0x384 {
                            let a = rd32(ped + PED_AUX);
                            or32(a + 0x50, 0x80);
                            let a = rd32(ped + PED_AUX);
                            or32(a + 0x50, 0x100);
                        }
                    }
                }
                late_rest(this, ped)
            }
        }

        // Middle path (0xD1E4AE on).
        #[inline(always)]
        unsafe fn mid(this: u32, ped: u32, sub: u32, w224: u32) -> u32 {
            unsafe {
                let mut d7out = [0u32; 4];
                let g: u32 = lf_checker_rt::callee_thiscall!(16, u32, this, ped, d7out.as_mut_ptr() as u32);
                if (g & 0xff) == 0 {
                    return late(this, ped);
                }
                if (rd8(sub + 0x0c) & 1) == 0 {
                    let r = icall3(sub, 0x14, ped, 1, 0);
                    if (r & 0xff) == 0 {
                        return late(this, ped);
                    }
                    or32(sub + 0x0c, 2);
                }
                wr32(this + 0xa0, d7out[0]);
                wr32(this + 0xa4, d7out[1]);
                wr32(this + 0xa8, d7out[2]);
                wr32(this + 0xac, d7out[3]);
                let pool = (lf_checker_rt::global::<u32>(POOL) as *const u32).read();
                let w: u32 = lf_checker_rt::callee_thiscall!(17, u32, pool);
                let _slot10 = if w == 0 {
                    0
                } else {
                    let f98 = rdf(this + 0x98).to_bits();
                    lf_checker_rt::callee_thiscall!(18, u32, w, f98, this + 0xa0, 0x3dcccccd, 0x40400000, 0xffff_ffff, 1, 0, 0, 0x3f000000, 1)
                };
                let t10: u32 = lf_checker_rt::callee_thiscall!(19, u32, w224);
                let run_d11 = if (rd8(t10 + 0x8f4) & 0x1c) == 4 {
                    true
                } else {
                    let t11: u32 = lf_checker_rt::callee_thiscall!(19, u32, w224);
                    (rd8(t11 + 0x8f4) & 0x1c) == 0
                };
                if run_d11 {
                    let n: u32 = lf_checker_rt::callee_cdecl!(20, u32,);
                    let c0 = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe8800)));
                    let k = f32::from_bits(rd32(lf_checker_rt::relocated(0x00fe8684)));
                    let prod = mul((n as i32) as f32, k);
                    if prod < c0 {
                        lf_checker_rt::callee_thiscall!(21, u32, ped, 1, 0x3e8);
                    }
                }
                let pos = rd32(ped + PED_POS);
                let e18 = subf(f32::from_bits(d7out[0]), rdf(pos + 0x30));
                let e14 = subf(f32::from_bits(d7out[1]), rdf(pos + 0x34));
                let e1c = subf(f32::from_bits(d7out[2]), rdf(pos + 0x38));
                let tag2 = rd32(ped + PED_TAG);
                let mut o40 = [0u32; 3];
                let mut o50 = [
                    subf(rdf(this + 0x20), rdf(pos + 0x30)).to_bits(),
                    subf(rdf(this + 0x24), rdf(pos + 0x34)).to_bits(),
                    subf(rdf(this + 0x28), rdf(pos + 0x38)).to_bits(),
                ];
                lf_checker_rt::callee_thiscall!(22, u32, tag2, o40.as_mut_ptr() as u32, o50.as_mut_ptr() as u32);
                let mut obyte = 0u32;
                let mut vecdum = [0u32; 3];
                let _fret: u32 = lf_checker_rt::callee_cdecl!(23, u32, ped, vecdum.as_mut_ptr() as u32, &mut obyte as *mut u32 as u32);
                let q0 = mul(e18, e18);
                let q1 = mul(e14, e14);
                let q2 = add(q1, q0);
                let q3 = mul(e1c, e1c);
                let q4 = add(q2, q3);
                let rt = core::hint::black_box(q4).sqrt();
                lf_checker_rt::callee_thiscall!(24, u32, this, ped, rt.to_bits(), obyte);
                let w2: u32 = lf_checker_rt::callee_thiscall!(17, u32, pool);
                if w2 == 0 {
                    return 0;
                }
                let i2b = icall3(this, 0x54, 0xea60, 0, 0);
                lf_checker_rt::callee_thiscall!(25, u32, ped, e18.to_bits(), i2b)
            }
        }

        // Main body.
        let sub = rd32(this + SUB);
        if icall0(sub, STATE_SLOT) == 0x2ca {
            return rd32(this + SUB);
        }
        let w224 = rd32(ped + PED_EXTRA);
        let mval = rd32(w224 + 0x264) as i32;
        let clim = rd32(lf_checker_rt::relocated(0x00e9d7fc)) as i32;
        if mval >= clim {
            let a = icall0(this, STATE_SLOT);
            let mut t1 = [0u32; 4];
            lf_checker_rt::callee_thiscall!(10, u32, t1.as_mut_ptr() as u32, a);
            let mut t2 = [0u32; 4];
            lf_checker_rt::callee_thiscall!(11, u32, w224.wrapping_add(0x84), t2.as_mut_ptr() as u32, 0, 1);
            let mut t3 = [0u32; 4];
            lf_checker_rt::callee_thiscall!(12, u32, t3.as_mut_ptr() as u32);
            return rd32(this + SUB);
        }
        or32(ped + 0x29c, 0x10000000);
        let tag = rd32(ped + PED_TAG);
        let via_checks = tag == 0 || rd8(this + 0x60) != 0 || rd8(this + 0x6c) != 0;
        if via_checks {
            if icall0(sub, STATE_SLOT) == 0x119 {
                return mid(this, ped, sub, w224);
            }
            if icall0(sub, STATE_SLOT) == 0x76d {
                return mid(this, ped, sub, w224);
            }
            if icall0(sub, STATE_SLOT) == 0x41a {
                return mid(this, ped, sub, w224);
            }
        }
        let tag2 = rd32(ped + PED_TAG);
        if tag2 == 0 || rd8(this + 0x60) != 0 || rd8(this + 0x6c) != 0 {
            return late(this, ped);
        }
        let d4: u32 = lf_checker_rt::callee_thiscall!(13, u32, tag2);
        if (d4 & 0xff) == 0 && ((rd32(rd32(ped + PED_TAG)) >> 11) & 1) == 0 {
            return late(this, ped);
        }
        let d5: u32 = lf_checker_rt::callee_thiscall!(14, u32, sub, ped, 1, 0);
        if (d5 & 0xff) == 0 {
            return late(this, ped);
        }
        lf_checker_rt::callee_thiscall!(15, u32, ped);
        icall3(this, 0x54, 0x3e8, 0, 0)
    }
});
