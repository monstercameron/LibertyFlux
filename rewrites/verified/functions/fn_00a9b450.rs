// original: 0x00A9B450 files-memory spatial resample (unnamed in symbols)
//! Full rewrite: entry frame, transform call, float block, sphere loop
//! over the entry list, primer loop over unprimed lists, tail block,
//! counter checks, the second per-node loop (normalise, stepped store,
//! RNG scatter, conditional fixup call) and the scatter tail with its
//! conditional global-vtable call, then the full epilogue.
lf_checker_rt::export!(thiscall, aq09_fn3(this: u32, arg0: u32, arg1: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_XFORM: u32 = 1; // matrix transform (thiscall/1, frame blocks)
    const CAL_COOKIE: u32 = 2; // frame-cookie check (cdecl/0)
    const CAL_COUNT: u32 = 3; // entry counter (thiscall/0)
    const CAL_BLEND: u32 = 4; // vector blend (cdecl/6, frame out-params)
    const CAL_PRIMER: u32 = 5; // node primer (thiscall/18)
    const CAL_UNLINK: u32 = 6; // list unlink (thiscall/1, ecx is list head + 8)
    const CAL_NOTIFY: u32 = 7; // node notify (thiscall/1, ecx is a global object)
    const CAL_FIXUP: u32 = 8; // list fixup (thiscall/2, ecx is a global object)
    // Note: contract id 9 is the planted global-vtable slot 0x14, called
    // through the fabricated object (vcall1), not through the stub table.

    // Globals (file VAs; resolved through the worker's image base).
    const RNG_LO_VA: u32 = 0x011101A0; // RNG low word (read + written)
    const RNG_HI_VA: u32 = 0x011101A4; // RNG high word (read + written)
    const K_BLEND_VA: u32 = 0x00FE864C; // blend-factor scale constant
    const NOTIFY_OBJ_VA: u32 = 0x01305D30; // notify callee's object
    const FIXUP_OBJ_VA: u32 = 0x013BABA0; // fixup callee's object
    const RNG_MULT: u32 = 0x5CDCFAA7; // RNG multiply step
    const K_GEOM_VA: u32 = 0x00E9A400; // node-geometry scale (one third)
    const K_ONE_VA: u32 = 0x00FE88E8; // float one
    const K_STEP_VA: u32 = 0x00FE8B68; // step scale (fifty)
    const K_SCAT0_VA: u32 = 0x00FE8830; // scatter radius scale (one half)
    const K_SCAT1_VA: u32 = 0x00FE8A24; // scatter direction scale (two)
    const K_NEG_VA: u32 = 0x00FE8FA0; // sign-flip mask
    const K_TAIL_VA: u32 = 0x00E981B0; // scatter-tail limit
    const CTR_VA: u32 = 0x011735B4; // frame counter (read-only)
    const VT_OBJ_VA: u32 = 0x018B8968; // global holding the scatter object

    /// Scalar float add with the original's exact NaN propagation: a NaN
    /// destination wins (quieted), else a NaN source wins (quieted);
    /// otherwise a plain add, which is order-free without NaNs. Never
    /// inlined, so the operation order of a chain cannot be reassociated.
    #[inline(never)]
    fn fadd_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest + src
    }

    /// Scalar float multiply with the same NaN rule as fadd_ss above.
    #[inline(never)]
    fn fmul_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest * src
    }

    /// Scalar float subtract with the same NaN rule as fadd_ss above.
    #[inline(never)]
    fn fsub_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest - src
    }

    /// Scalar float divide with the same NaN rule as fadd_ss above.
    #[inline(never)]
    fn fdiv_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest / src
    }

    /// One RNG step: 64-bit multiply-add over the shared pair, returning
    /// the low 23 bits of the new low word as an exact float.
    #[inline(always)]
    unsafe fn rng_draw() -> f32 {
        let lo = *(lf_checker_rt::global::<u32>(RNG_LO_VA) as *const u32);
        let hi = *(lf_checker_rt::global::<u32>(RNG_HI_VA) as *const u32);
        let step = (lo as u64)
            .wrapping_mul(RNG_MULT as u64)
            .wrapping_add(hi as u64);
        *(lf_checker_rt::global::<u32>(RNG_LO_VA)) = step as u32;
        *(lf_checker_rt::global::<u32>(RNG_HI_VA)) = (step >> 32) as u32;
        ((step as u32) & 0x007FFFFF) as f32
    }

    /// Call a planted vtable slot exactly like the original: load the
    /// table pointer from the object, load the slot, call it. Both sides
    /// land on the same recorder stub.
    #[inline(always)]
    unsafe fn vcall1(object: u32, slot: u32, arg: u32) -> u32 {
        let vtable = *(object as *const u32);
        let target = *((vtable.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object, arg)
    }

    unsafe {
        // Entry: owner -> matrix object, 12 matrix floats (the source
        // has no fourth column; frame slots without a source read the
        // defined stack fill, 0.0, exactly as the original does).
        let o68 = *((arg0.wrapping_add(0x68)) as *const u32);
        let mat = *((o68.wrapping_add(0x20)) as *const u32);
        let m00 = *((mat.wrapping_add(0x00)) as *const f32);
        let m04 = *((mat.wrapping_add(0x04)) as *const f32);
        let m08 = *((mat.wrapping_add(0x08)) as *const f32);
        let m10 = *((mat.wrapping_add(0x10)) as *const f32);
        let m14 = *((mat.wrapping_add(0x14)) as *const f32);
        let m18 = *((mat.wrapping_add(0x18)) as *const f32);
        let m20 = *((mat.wrapping_add(0x20)) as *const f32);
        let m24 = *((mat.wrapping_add(0x24)) as *const f32);
        let m28 = *((mat.wrapping_add(0x28)) as *const f32);
        let m30 = *((mat.wrapping_add(0x30)) as *const f32);
        let m34 = *((mat.wrapping_add(0x34)) as *const f32);
        let m38 = *((mat.wrapping_add(0x38)) as *const f32);
        // Both blocks start as the same 4x4 matrix with an empty fourth
        // column. Note: the stores after the first push address one word
        // lower than their displacement says.
        let src = [
            m00, m04, m08, 0.0, m10, m14, m18, 0.0, m20, m24, m28, 0.0, m30,
            m34, m38, 0.0,
        ];
        let mut dst = [
            m00, m04, m08, 0.0, m10, m14, m18, 0.0, m20, m24, m28, 0.0, m30,
            m34, m38, 0.0,
        ];
        // Transform: the callee fills the block's first row.
        lf_checker_rt::callee_thiscall!(
            CAL_XFORM,
            u32,
            dst.as_mut_ptr() as u32,
            src.as_ptr() as u32
        );
        // Context vector, then the three dot products below.
        let e10 = *((arg1.wrapping_add(0x10)) as *const f32);
        let e14 = *((arg1.wrapping_add(0x14)) as *const f32);
        let e18 = *((arg1.wrapping_add(0x18)) as *const f32);
        // Float block: three dot products over the block and the
        // context vector, one scalar operation per statement in the
        // original's exact order (see fadd_ss).
        let a40 = fmul_ss(dst[4], e14);
        let b40 = fmul_ss(dst[0], e10);
        let s40 = fadd_ss(a40, b40);
        let c40 = fmul_ss(dst[8], e18);
        let t40 = fadd_ss(s40, c40);
        let f40 = fadd_ss(t40, dst[12]);
        let a2c = fmul_ss(dst[5], e14);
        let b2c = fmul_ss(dst[1], e10);
        let s2c = fadd_ss(a2c, b2c);
        let c2c = fmul_ss(dst[9], e18);
        let t2c = fadd_ss(s2c, c2c);
        let f2c = fadd_ss(t2c, dst[13]);
        let a20 = fmul_ss(dst[6], e14);
        let b20 = fmul_ss(dst[2], e10);
        let s20 = fadd_ss(a20, b20);
        let c20 = fmul_ss(dst[10], e18);
        let t20 = fadd_ss(s20, c20);
        let f20 = fadd_ss(t20, dst[14]);
        // Sphere loop over the entry list: each flagged node carries
        // three spheres, and a radius covering the point exits early.
        let mut use_hit_path = false;
        let mut cur = *((this.wrapping_add(0x0C)) as *const u32);
        while cur != 0 {
            let flagged = *((cur.wrapping_add(0xF0)) as *const u8) != 0;
            let next = *(cur as *const u32);
            if flagged {
                let r = *((arg1.wrapping_add(0x20)) as *const f32);
                let r2 = fmul_ss(r, r);
                let mut k = 0u32;
                while k < 3 {
                    let base = cur.wrapping_add(0x10).wrapping_add(k.wrapping_mul(0x10));
                    let dx = fsub_ss(*(base as *const f32), f40);
                    let dy = fsub_ss(
                        *((base.wrapping_add(4)) as *const f32),
                        f2c,
                    );
                    let dz = fsub_ss(
                        *((base.wrapping_add(8)) as *const f32),
                        f20,
                    );
                    let dx2 = fmul_ss(dx, dx);
                    let dy2 = fmul_ss(dy, dy);
                    let dz2 = fmul_ss(dz, dz);
                    let d2 = fadd_ss(fadd_ss(dx2, dy2), dz2);
                    // Ordered greater-or-equal, matching comiss+jae.
                    if r2 >= d2 {
                        use_hit_path = true;
                        break;
                    }
                    k = k.wrapping_add(1);
                }
                if use_hit_path {
                    break;
                }
            }
            cur = next;
        }
        if !use_hit_path {
            let flag85 = *((arg0.wrapping_add(0x85)) as *const u8);
            if flag85 == 0 {
                // Early epilogue.
                lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
                return 0;
            }
        }
        // Primer loop: on an unprimed list, walk the entry list head,
        // drawing one RNG value per node, blending a per-node factor
        // from the owner's range, priming the node, unlinking it and
        // notifying, then mark the list primed.
        if *((this.wrapping_add(0x14)) as *const u8) == 0 {
            let mut prim = *((this.wrapping_add(0x0C)) as *const u32);
            if prim != 0 {
                let range_lo = *((arg0.wrapping_add(0x88)) as *const f32);
                let range_hi = *((arg0.wrapping_add(0x8C)) as *const f32);
                let range_span = fsub_ss(range_hi, range_lo);
                let blend_scale =
                    *(lf_checker_rt::global::<f32>(K_BLEND_VA) as *const f32);
                loop {
                    let next = *(prim as *const u32);
                    let frac = rng_draw();
                    let blend = fadd_ss(
                        fmul_ss(fmul_ss(frac, blend_scale), range_span),
                        range_lo,
                    );
                    // Prime call: node interior pointers, the owner,
                    // the blend factor and a cleared out-slot.
                    let mut out_slot = 0u32;
                    lf_checker_rt::callee_thiscall!(
                        CAL_PRIMER,
                        u32,
                        this,
                        prim.wrapping_add(0x10),
                        prim.wrapping_add(0x20),
                        prim.wrapping_add(0x30),
                        prim.wrapping_add(0x40),
                        prim.wrapping_add(0x50),
                        prim.wrapping_add(0x60),
                        prim.wrapping_add(0x70),
                        prim.wrapping_add(0x78),
                        prim.wrapping_add(0x80),
                        prim.wrapping_add(0x88),
                        prim.wrapping_add(0x8C),
                        prim.wrapping_add(0x90),
                        prim.wrapping_add(0xA0),
                        prim.wrapping_add(0xB0),
                        prim.wrapping_add(0xC0),
                        arg0,
                        blend.to_bits(),
                        &mut out_slot as *mut u32 as u32,
                    );
                    lf_checker_rt::callee_thiscall!(
                        CAL_UNLINK,
                        u32,
                        this.wrapping_add(8),
                        prim
                    );
                    lf_checker_rt::callee_thiscall!(
                        CAL_NOTIFY,
                        u32,
                        lf_checker_rt::relocated(NOTIFY_OBJ_VA),
                        prim
                    );
                    prim = next;
                    if prim == 0 {
                        break;
                    }
                }
            }
            *((this.wrapping_add(0x14)) as *mut u8) = 1;
        }
        // Tail block. The scratch slot below the blend outputs keeps
        // the fourth blend word on blend rows and the stack fill
        // (0.0) otherwise; the second loop stores it per node.
        let slot3c: f32;
        {
            let c0 = *((this.wrapping_add(0xC0)) as *const f32);
            if c0 == 0.0 {
                // Store variant: publish the three products.
                *((this.wrapping_add(0xB0)) as *mut f32) = f40;
                *((this.wrapping_add(0xB4)) as *mut f32) = f2c;
                *((this.wrapping_add(0xB8)) as *mut f32) = f20;
                *((this.wrapping_add(0xBC)) as *mut f32) = 0.0;
                let e20 = *((arg1.wrapping_add(0x20)) as *const u32);
                *((this.wrapping_add(0xC0)) as *mut u32) = e20;
                slot3c = 0.0;
            } else {
                // Blend-call variant: the callee fills the outputs
                // through frame pointers; its float arguments are the
                // current marker and context words (the original
                // overwrites two pushed slots with them).
                let mut out = [0.0f32; 4];
                let mut mark = 0.0f32;
                let c0bits = *((this.wrapping_add(0xC0)) as *const u32);
                let e20bits = *((arg1.wrapping_add(0x20)) as *const u32);
                lf_checker_rt::callee_cdecl!(
                    CAL_BLEND,
                    u32,
                    this.wrapping_add(0xB0),
                    c0bits,
                    &f40 as *const f32 as u32,
                    e20bits,
                    out.as_mut_ptr() as u32,
                    &mut mark as *mut f32 as u32,
                );
                *((this.wrapping_add(0xB0)) as *mut f32) = out[0];
                *((this.wrapping_add(0xB4)) as *mut f32) = out[1];
                *((this.wrapping_add(0xB8)) as *mut f32) = out[2];
                *((this.wrapping_add(0xBC)) as *mut f32) = out[3];
                *((this.wrapping_add(0xC0)) as *mut f32) = mark;
                slot3c = out[3];
            }
            // Counter checks: an over-limit slot count or entry
            // count raises the sticky flag (unsigned, then signed).
            if *((this.wrapping_add(8)) as *const u32) > 0x80 {
                *((this.wrapping_add(0x15)) as *mut u8) = 1;
            }
            let n = lf_checker_rt::callee_thiscall!(CAL_COUNT, u32, arg0);
            if (n as i32) > 0x200 {
                *((this.wrapping_add(0x15)) as *mut u8) = 1;
            }
            // Second list loop: per-node normalise, stepped store,
            // RNG scatter and conditional fixup call.
            let geom_k =
                *(lf_checker_rt::global::<f32>(K_GEOM_VA) as *const f32);
            let one = *(lf_checker_rt::global::<f32>(K_ONE_VA) as *const f32);
            let step_k =
                *(lf_checker_rt::global::<f32>(K_STEP_VA) as *const f32);
            let scat0 =
                *(lf_checker_rt::global::<f32>(K_SCAT0_VA) as *const f32);
            let scat1 =
                *(lf_checker_rt::global::<f32>(K_SCAT1_VA) as *const f32);
            let blend_k =
                *(lf_checker_rt::global::<f32>(K_BLEND_VA) as *const f32);
            let neg_mask =
                *(lf_checker_rt::global::<u32>(K_NEG_VA) as *const u32);
            let counter =
                lf_checker_rt::global::<u32>(CTR_VA) as *const u32;
            let mut processed = 0u32;
            let mut cur2 = *((this.wrapping_add(0x0C)) as *const u32);
            if cur2 != 0 {
                loop {
                    let live = *((cur2.wrapping_add(0xF0)) as *const u8) != 0;
                    let next2 = *(cur2 as *const u32);
                    let done = *((cur2.wrapping_add(0xF4)) as *const u32) != 0;
                    if live && !done {
                        // Node geometry, scaled and rebased onto the
                        // three dot products, one operation per
                        // statement in the original's exact order.
                        let n10 = *((cur2.wrapping_add(0x10)) as *const f32);
                        let n14 = *((cur2.wrapping_add(0x14)) as *const f32);
                        let n18 = *((cur2.wrapping_add(0x18)) as *const f32);
                        let n20 = *((cur2.wrapping_add(0x20)) as *const f32);
                        let n24 = *((cur2.wrapping_add(0x24)) as *const f32);
                        let n28 = *((cur2.wrapping_add(0x28)) as *const f32);
                        let n30 = *((cur2.wrapping_add(0x30)) as *const f32);
                        let n34 = *((cur2.wrapping_add(0x34)) as *const f32);
                        let n38 = *((cur2.wrapping_add(0x38)) as *const f32);
                        let gv4 = fsub_ss(
                            fmul_ss(fadd_ss(fadd_ss(n20, n10), n30), geom_k),
                            f40,
                        );
                        let gv2 = fsub_ss(
                            fmul_ss(fadd_ss(n34, fadd_ss(n14, n24)), geom_k),
                            f2c,
                        );
                        let gv3 = fsub_ss(
                            fmul_ss(fadd_ss(n38, fadd_ss(n18, n28)), geom_k),
                            f20,
                        );
                        // Matrix rows dotted with the geometry vector.
                        let uu = fadd_ss(
                            fadd_ss(fmul_ss(m10, gv2), fmul_ss(m00, gv4)),
                            fmul_ss(m20, gv3),
                        );
                        let vv = fadd_ss(
                            fadd_ss(fmul_ss(m14, gv2), fmul_ss(m04, gv4)),
                            fmul_ss(m24, gv3),
                        );
                        let ww = fadd_ss(
                            fadd_ss(fmul_ss(m18, gv2), fmul_ss(m08, gv4)),
                            fmul_ss(m28, gv3),
                        );
                        // Normalise scale: the compare idiom jumps unless
                        // the squared length is ordered-equal to zero,
                        // so only a zero length yields zero; anything
                        // else (NaN included) yields one over its root.
                        let n2 = fadd_ss(
                            fadd_ss(fmul_ss(vv, vv), fmul_ss(uu, uu)),
                            fmul_ss(ww, ww),
                        );
                        let root = n2.sqrt();
                        let scale = if n2 == 0.0 {
                            0.0
                        } else {
                            fdiv_ss(one, n2.sqrt())
                        };
                        let uu = fmul_ss(uu, scale);
                        let vv = fmul_ss(vv, scale);
                        let ww = fmul_ss(ww, scale);
                        // Radius gap, clamped at zero unless ordered
                        // greater (matching comiss+ja).
                        let erad = *((arg1.wrapping_add(0x20)) as *const f32);
                        let mut gap = fsub_ss(root, erad);
                        if !(gap > 0.0) {
                            gap = 0.0;
                        }
                        let no_flag =
                            *((arg0.wrapping_add(0x85)) as *const u8) == 0
                                && *((this.wrapping_add(0x15)) as *const u8)
                                    == 0;
                        if no_flag && gap > 0.0 {
                            // Positive gap with both flags clear skips
                            // the node: the scatter check below would
                            // find its stamp still zero.
                        } else {
                            let stamp = if no_flag {
                                // Non-positive gap: stamp the counter.
                                *counter
                            } else {
                                // Stepped stamp: chop the scaled gap
                                // to 64 bits (an out-of-range or NaN
                                // input stores the indefinite, whose
                                // low word is zero) and add the counter.
                                let t = fmul_ss(gap, step_k);
                                let chopped = if t.is_nan()
                                    || t >= 9.223372036854776e18f32
                                    || t <= -9.223372036854776e18f32
                                {
                                    0u32
                                } else {
                                    (t as i64) as u32
                                };
                                chopped.wrapping_add(*counter)
                            };
                            processed = processed.wrapping_add(1);
                            *((cur2.wrapping_add(0xF4)) as *mut u32) = stamp;
                            // Scaled store of the normal and the slot.
                            let e24 =
                                *((arg1.wrapping_add(0x24)) as *const f32);
                            let su = fmul_ss(uu, e24);
                            let sv = fmul_ss(vv, e24);
                            let sw = fmul_ss(ww, e24);
                            *((cur2.wrapping_add(0xDC)) as *mut f32) = slot3c;
                            *((cur2.wrapping_add(0xD8)) as *mut f32) = sw;
                            *((cur2.wrapping_add(0xD4)) as *mut f32) = sv;
                            *((cur2.wrapping_add(0xD0)) as *mut f32) = su;
                            if stamp != 0 {
                                // Scatter: three jittered components
                                // around a scaled radius.
                                let s2 = fadd_ss(
                                    fadd_ss(
                                        fmul_ss(su, su),
                                        fmul_ss(sv, sv),
                                    ),
                                    fmul_ss(sw, sw),
                                );
                                let g = fmul_ss(s2.sqrt(), scat0);
                                let neg =
                                    f32::from_bits(g.to_bits() ^ neg_mask);
                                let h = fsub_ss(g, neg);
                                let r0 = rng_draw();
                                let c0j = fadd_ss(
                                    fadd_ss(fmul_ss(fmul_ss(r0, blend_k), h), neg),
                                    su,
                                );
                                *((cur2.wrapping_add(0xD0)) as *mut f32) = c0j;
                                let r1 = rng_draw();
                                let c1j = fadd_ss(
                                    fadd_ss(fmul_ss(fmul_ss(r1, blend_k), h), neg),
                                    sv,
                                );
                                *((cur2.wrapping_add(0xD4)) as *mut f32) = c1j;
                                let r2 = rng_draw();
                                let c2j = fadd_ss(
                                    fadd_ss(fmul_ss(fmul_ss(r2, blend_k), h), neg),
                                    sw,
                                );
                                *((cur2.wrapping_add(0xD8)) as *mut f32) = c2j;
                                // Three scattered direction components.
                                let r3 = rng_draw();
                                let d0 = fsub_ss(
                                    fmul_ss(fmul_ss(r3, blend_k), scat1),
                                    one,
                                );
                                *((cur2.wrapping_add(0xE0)) as *mut f32) = d0;
                                let r4 = rng_draw();
                                let d1 = fsub_ss(
                                    fmul_ss(fmul_ss(r4, blend_k), scat1),
                                    one,
                                );
                                *((cur2.wrapping_add(0xE4)) as *mut f32) = d1;
                                let r5 = rng_draw();
                                let d2 = fsub_ss(
                                    fmul_ss(fmul_ss(r5, blend_k), scat1),
                                    one,
                                );
                                *((cur2.wrapping_add(0xE8)) as *mut f32) = d2;
                                // Normalise the scattered direction.
                                let q2 = fadd_ss(
                                    fadd_ss(
                                        fmul_ss(d0, d0),
                                        fmul_ss(d1, d1),
                                    ),
                                    fmul_ss(d2, d2),
                                );
                                let scale2 = if q2 == 0.0 {
                                    0.0
                                } else {
                                    fdiv_ss(one, q2.sqrt())
                                };
                                *((cur2.wrapping_add(0xE0)) as *mut f32) =
                                    fmul_ss(scale2, d0);
                                *((cur2.wrapping_add(0xE4)) as *mut f32) =
                                    fmul_ss(scale2, d1);
                                *((cur2.wrapping_add(0xE8)) as *mut f32) =
                                    fmul_ss(scale2, d2);
                                // One fixup call until the gate clears.
                                if *((this.wrapping_add(0x18)) as *const u32)
                                    != 0
                                {
                                    let a68 = *((arg0.wrapping_add(0x68))
                                        as *const u32);
                                    lf_checker_rt::callee_thiscall!(
                                        CAL_FIXUP,
                                        u32,
                                        lf_checker_rt::relocated(FIXUP_OBJ_VA),
                                        a68,
                                        this
                                    );
                                    *((this.wrapping_add(0x18)) as *mut u32) =
                                        0;
                                }
                            }
                        }
                    }
                    cur2 = next2;
                    if cur2 == 0 {
                        break;
                    }
                }
            }
            // Scatter tail: a fresh-enough processed list notifies
            // through the global object's vtable and stamps the age.
            if (processed as i32) > 0 {
                let tail_k =
                    *(lf_checker_rt::global::<f32>(K_TAIL_VA) as *const f32);
                let a78 = *((arg0.wrapping_add(0x78)) as *const f32);
                if tail_k > a78 {
                    let ctr_now = *counter;
                    let age = ctr_now
                        .wrapping_sub(*((this.wrapping_add(0xC4)) as *const u32));
                    if age > 0x1F4 {
                        let robj = *(lf_checker_rt::global::<u32>(VT_OBJ_VA)
                            as *const u32);
                        let carg = *((arg0.wrapping_add(0x60)) as *const u32);
                        vcall1(robj, 0x14, carg);
                        *((this.wrapping_add(0xC4)) as *mut u32) = ctr_now;
                    }
                }
            }
            // Full epilogue: copy the context tag, then check.
            let tag = *((arg1.wrapping_add(0x28)) as *const u8);
            *((arg0.wrapping_add(0x84)) as *mut u8) = tag;
            lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }
    }
});

// Wrong version (mutant) of aq09_fn3: identical except the RNG
// multiply step ends in 0xA6 instead of 0xA7. Every primer and
// scatter draw then differs, so the same contract must reject it.
lf_checker_rt::export!(thiscall, aq09_fn3_mut(this: u32, arg0: u32, arg1: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_XFORM: u32 = 1; // matrix transform (thiscall/1, frame blocks)
    const CAL_COOKIE: u32 = 2; // frame-cookie check (cdecl/0)
    const CAL_COUNT: u32 = 3; // entry counter (thiscall/0)
    const CAL_BLEND: u32 = 4; // vector blend (cdecl/6, frame out-params)
    const CAL_PRIMER: u32 = 5; // node primer (thiscall/18)
    const CAL_UNLINK: u32 = 6; // list unlink (thiscall/1, ecx is list head + 8)
    const CAL_NOTIFY: u32 = 7; // node notify (thiscall/1, ecx is a global object)
    const CAL_FIXUP: u32 = 8; // list fixup (thiscall/2, ecx is a global object)
    // Note: contract id 9 is the planted global-vtable slot 0x14, called
    // through the fabricated object (vcall1), not through the stub table.

    // Globals (file VAs; resolved through the worker's image base).
    const RNG_LO_VA: u32 = 0x011101A0; // RNG low word (read + written)
    const RNG_HI_VA: u32 = 0x011101A4; // RNG high word (read + written)
    const K_BLEND_VA: u32 = 0x00FE864C; // blend-factor scale constant
    const NOTIFY_OBJ_VA: u32 = 0x01305D30; // notify callee's object
    const FIXUP_OBJ_VA: u32 = 0x013BABA0; // fixup callee's object
    const RNG_MULT: u32 = 0x5CDCFAA6; // RNG multiply step
    const K_GEOM_VA: u32 = 0x00E9A400; // node-geometry scale (one third)
    const K_ONE_VA: u32 = 0x00FE88E8; // float one
    const K_STEP_VA: u32 = 0x00FE8B68; // step scale (fifty)
    const K_SCAT0_VA: u32 = 0x00FE8830; // scatter radius scale (one half)
    const K_SCAT1_VA: u32 = 0x00FE8A24; // scatter direction scale (two)
    const K_NEG_VA: u32 = 0x00FE8FA0; // sign-flip mask
    const K_TAIL_VA: u32 = 0x00E981B0; // scatter-tail limit
    const CTR_VA: u32 = 0x011735B4; // frame counter (read-only)
    const VT_OBJ_VA: u32 = 0x018B8968; // global holding the scatter object

    /// Scalar float add with the original's exact NaN propagation: a NaN
    /// destination wins (quieted), else a NaN source wins (quieted);
    /// otherwise a plain add, which is order-free without NaNs. Never
    /// inlined, so the operation order of a chain cannot be reassociated.
    #[inline(never)]
    fn fadd_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest + src
    }

    /// Scalar float multiply with the same NaN rule as fadd_ss above.
    #[inline(never)]
    fn fmul_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest * src
    }

    /// Scalar float subtract with the same NaN rule as fadd_ss above.
    #[inline(never)]
    fn fsub_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest - src
    }

    /// Scalar float divide with the same NaN rule as fadd_ss above.
    #[inline(never)]
    fn fdiv_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest / src
    }

    /// One RNG step: 64-bit multiply-add over the shared pair, returning
    /// the low 23 bits of the new low word as an exact float.
    #[inline(always)]
    unsafe fn rng_draw() -> f32 {
        let lo = *(lf_checker_rt::global::<u32>(RNG_LO_VA) as *const u32);
        let hi = *(lf_checker_rt::global::<u32>(RNG_HI_VA) as *const u32);
        let step = (lo as u64)
            .wrapping_mul(RNG_MULT as u64)
            .wrapping_add(hi as u64);
        *(lf_checker_rt::global::<u32>(RNG_LO_VA)) = step as u32;
        *(lf_checker_rt::global::<u32>(RNG_HI_VA)) = (step >> 32) as u32;
        ((step as u32) & 0x007FFFFF) as f32
    }

    /// Call a planted vtable slot exactly like the original: load the
    /// table pointer from the object, load the slot, call it. Both sides
    /// land on the same recorder stub.
    #[inline(always)]
    unsafe fn vcall1(object: u32, slot: u32, arg: u32) -> u32 {
        let vtable = *(object as *const u32);
        let target = *((vtable.wrapping_add(slot)) as *const u32);
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(object, arg)
    }

    unsafe {
        // Entry: owner -> matrix object, 12 matrix floats (the source
        // has no fourth column; frame slots without a source read the
        // defined stack fill, 0.0, exactly as the original does).
        let o68 = *((arg0.wrapping_add(0x68)) as *const u32);
        let mat = *((o68.wrapping_add(0x20)) as *const u32);
        let m00 = *((mat.wrapping_add(0x00)) as *const f32);
        let m04 = *((mat.wrapping_add(0x04)) as *const f32);
        let m08 = *((mat.wrapping_add(0x08)) as *const f32);
        let m10 = *((mat.wrapping_add(0x10)) as *const f32);
        let m14 = *((mat.wrapping_add(0x14)) as *const f32);
        let m18 = *((mat.wrapping_add(0x18)) as *const f32);
        let m20 = *((mat.wrapping_add(0x20)) as *const f32);
        let m24 = *((mat.wrapping_add(0x24)) as *const f32);
        let m28 = *((mat.wrapping_add(0x28)) as *const f32);
        let m30 = *((mat.wrapping_add(0x30)) as *const f32);
        let m34 = *((mat.wrapping_add(0x34)) as *const f32);
        let m38 = *((mat.wrapping_add(0x38)) as *const f32);
        // Both blocks start as the same 4x4 matrix with an empty fourth
        // column. Note: the stores after the first push address one word
        // lower than their displacement says.
        let src = [
            m00, m04, m08, 0.0, m10, m14, m18, 0.0, m20, m24, m28, 0.0, m30,
            m34, m38, 0.0,
        ];
        let mut dst = [
            m00, m04, m08, 0.0, m10, m14, m18, 0.0, m20, m24, m28, 0.0, m30,
            m34, m38, 0.0,
        ];
        // Transform: the callee fills the block's first row.
        lf_checker_rt::callee_thiscall!(
            CAL_XFORM,
            u32,
            dst.as_mut_ptr() as u32,
            src.as_ptr() as u32
        );
        // Context vector, then the three dot products below.
        let e10 = *((arg1.wrapping_add(0x10)) as *const f32);
        let e14 = *((arg1.wrapping_add(0x14)) as *const f32);
        let e18 = *((arg1.wrapping_add(0x18)) as *const f32);
        // Float block: three dot products over the block and the
        // context vector, one scalar operation per statement in the
        // original's exact order (see fadd_ss).
        let a40 = fmul_ss(dst[4], e14);
        let b40 = fmul_ss(dst[0], e10);
        let s40 = fadd_ss(a40, b40);
        let c40 = fmul_ss(dst[8], e18);
        let t40 = fadd_ss(s40, c40);
        let f40 = fadd_ss(t40, dst[12]);
        let a2c = fmul_ss(dst[5], e14);
        let b2c = fmul_ss(dst[1], e10);
        let s2c = fadd_ss(a2c, b2c);
        let c2c = fmul_ss(dst[9], e18);
        let t2c = fadd_ss(s2c, c2c);
        let f2c = fadd_ss(t2c, dst[13]);
        let a20 = fmul_ss(dst[6], e14);
        let b20 = fmul_ss(dst[2], e10);
        let s20 = fadd_ss(a20, b20);
        let c20 = fmul_ss(dst[10], e18);
        let t20 = fadd_ss(s20, c20);
        let f20 = fadd_ss(t20, dst[14]);
        // Sphere loop over the entry list: each flagged node carries
        // three spheres, and a radius covering the point exits early.
        let mut use_hit_path = false;
        let mut cur = *((this.wrapping_add(0x0C)) as *const u32);
        while cur != 0 {
            let flagged = *((cur.wrapping_add(0xF0)) as *const u8) != 0;
            let next = *(cur as *const u32);
            if flagged {
                let r = *((arg1.wrapping_add(0x20)) as *const f32);
                let r2 = fmul_ss(r, r);
                let mut k = 0u32;
                while k < 3 {
                    let base = cur.wrapping_add(0x10).wrapping_add(k.wrapping_mul(0x10));
                    let dx = fsub_ss(*(base as *const f32), f40);
                    let dy = fsub_ss(
                        *((base.wrapping_add(4)) as *const f32),
                        f2c,
                    );
                    let dz = fsub_ss(
                        *((base.wrapping_add(8)) as *const f32),
                        f20,
                    );
                    let dx2 = fmul_ss(dx, dx);
                    let dy2 = fmul_ss(dy, dy);
                    let dz2 = fmul_ss(dz, dz);
                    let d2 = fadd_ss(fadd_ss(dx2, dy2), dz2);
                    // Ordered greater-or-equal, matching comiss+jae.
                    if r2 >= d2 {
                        use_hit_path = true;
                        break;
                    }
                    k = k.wrapping_add(1);
                }
                if use_hit_path {
                    break;
                }
            }
            cur = next;
        }
        if !use_hit_path {
            let flag85 = *((arg0.wrapping_add(0x85)) as *const u8);
            if flag85 == 0 {
                // Early epilogue.
                lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
                return 0;
            }
        }
        // Primer loop: on an unprimed list, walk the entry list head,
        // drawing one RNG value per node, blending a per-node factor
        // from the owner's range, priming the node, unlinking it and
        // notifying, then mark the list primed.
        if *((this.wrapping_add(0x14)) as *const u8) == 0 {
            let mut prim = *((this.wrapping_add(0x0C)) as *const u32);
            if prim != 0 {
                let range_lo = *((arg0.wrapping_add(0x88)) as *const f32);
                let range_hi = *((arg0.wrapping_add(0x8C)) as *const f32);
                let range_span = fsub_ss(range_hi, range_lo);
                let blend_scale =
                    *(lf_checker_rt::global::<f32>(K_BLEND_VA) as *const f32);
                loop {
                    let next = *(prim as *const u32);
                    let frac = rng_draw();
                    let blend = fadd_ss(
                        fmul_ss(fmul_ss(frac, blend_scale), range_span),
                        range_lo,
                    );
                    // Prime call: node interior pointers, the owner,
                    // the blend factor and a cleared out-slot.
                    let mut out_slot = 0u32;
                    lf_checker_rt::callee_thiscall!(
                        CAL_PRIMER,
                        u32,
                        this,
                        prim.wrapping_add(0x10),
                        prim.wrapping_add(0x20),
                        prim.wrapping_add(0x30),
                        prim.wrapping_add(0x40),
                        prim.wrapping_add(0x50),
                        prim.wrapping_add(0x60),
                        prim.wrapping_add(0x70),
                        prim.wrapping_add(0x78),
                        prim.wrapping_add(0x80),
                        prim.wrapping_add(0x88),
                        prim.wrapping_add(0x8C),
                        prim.wrapping_add(0x90),
                        prim.wrapping_add(0xA0),
                        prim.wrapping_add(0xB0),
                        prim.wrapping_add(0xC0),
                        arg0,
                        blend.to_bits(),
                        &mut out_slot as *mut u32 as u32,
                    );
                    lf_checker_rt::callee_thiscall!(
                        CAL_UNLINK,
                        u32,
                        this.wrapping_add(8),
                        prim
                    );
                    lf_checker_rt::callee_thiscall!(
                        CAL_NOTIFY,
                        u32,
                        lf_checker_rt::relocated(NOTIFY_OBJ_VA),
                        prim
                    );
                    prim = next;
                    if prim == 0 {
                        break;
                    }
                }
            }
            *((this.wrapping_add(0x14)) as *mut u8) = 1;
        }
        // Tail block. The scratch slot below the blend outputs keeps
        // the fourth blend word on blend rows and the stack fill
        // (0.0) otherwise; the second loop stores it per node.
        let slot3c: f32;
        {
            let c0 = *((this.wrapping_add(0xC0)) as *const f32);
            if c0 == 0.0 {
                // Store variant: publish the three products.
                *((this.wrapping_add(0xB0)) as *mut f32) = f40;
                *((this.wrapping_add(0xB4)) as *mut f32) = f2c;
                *((this.wrapping_add(0xB8)) as *mut f32) = f20;
                *((this.wrapping_add(0xBC)) as *mut f32) = 0.0;
                let e20 = *((arg1.wrapping_add(0x20)) as *const u32);
                *((this.wrapping_add(0xC0)) as *mut u32) = e20;
                slot3c = 0.0;
            } else {
                // Blend-call variant: the callee fills the outputs
                // through frame pointers; its float arguments are the
                // current marker and context words (the original
                // overwrites two pushed slots with them).
                let mut out = [0.0f32; 4];
                let mut mark = 0.0f32;
                let c0bits = *((this.wrapping_add(0xC0)) as *const u32);
                let e20bits = *((arg1.wrapping_add(0x20)) as *const u32);
                lf_checker_rt::callee_cdecl!(
                    CAL_BLEND,
                    u32,
                    this.wrapping_add(0xB0),
                    c0bits,
                    &f40 as *const f32 as u32,
                    e20bits,
                    out.as_mut_ptr() as u32,
                    &mut mark as *mut f32 as u32,
                );
                *((this.wrapping_add(0xB0)) as *mut f32) = out[0];
                *((this.wrapping_add(0xB4)) as *mut f32) = out[1];
                *((this.wrapping_add(0xB8)) as *mut f32) = out[2];
                *((this.wrapping_add(0xBC)) as *mut f32) = out[3];
                *((this.wrapping_add(0xC0)) as *mut f32) = mark;
                slot3c = out[3];
            }
            // Counter checks: an over-limit slot count or entry
            // count raises the sticky flag (unsigned, then signed).
            if *((this.wrapping_add(8)) as *const u32) > 0x80 {
                *((this.wrapping_add(0x15)) as *mut u8) = 1;
            }
            let n = lf_checker_rt::callee_thiscall!(CAL_COUNT, u32, arg0);
            if (n as i32) > 0x200 {
                *((this.wrapping_add(0x15)) as *mut u8) = 1;
            }
            // Second list loop: per-node normalise, stepped store,
            // RNG scatter and conditional fixup call.
            let geom_k =
                *(lf_checker_rt::global::<f32>(K_GEOM_VA) as *const f32);
            let one = *(lf_checker_rt::global::<f32>(K_ONE_VA) as *const f32);
            let step_k =
                *(lf_checker_rt::global::<f32>(K_STEP_VA) as *const f32);
            let scat0 =
                *(lf_checker_rt::global::<f32>(K_SCAT0_VA) as *const f32);
            let scat1 =
                *(lf_checker_rt::global::<f32>(K_SCAT1_VA) as *const f32);
            let blend_k =
                *(lf_checker_rt::global::<f32>(K_BLEND_VA) as *const f32);
            let neg_mask =
                *(lf_checker_rt::global::<u32>(K_NEG_VA) as *const u32);
            let counter =
                lf_checker_rt::global::<u32>(CTR_VA) as *const u32;
            let mut processed = 0u32;
            let mut cur2 = *((this.wrapping_add(0x0C)) as *const u32);
            if cur2 != 0 {
                loop {
                    let live = *((cur2.wrapping_add(0xF0)) as *const u8) != 0;
                    let next2 = *(cur2 as *const u32);
                    let done = *((cur2.wrapping_add(0xF4)) as *const u32) != 0;
                    if live && !done {
                        // Node geometry, scaled and rebased onto the
                        // three dot products, one operation per
                        // statement in the original's exact order.
                        let n10 = *((cur2.wrapping_add(0x10)) as *const f32);
                        let n14 = *((cur2.wrapping_add(0x14)) as *const f32);
                        let n18 = *((cur2.wrapping_add(0x18)) as *const f32);
                        let n20 = *((cur2.wrapping_add(0x20)) as *const f32);
                        let n24 = *((cur2.wrapping_add(0x24)) as *const f32);
                        let n28 = *((cur2.wrapping_add(0x28)) as *const f32);
                        let n30 = *((cur2.wrapping_add(0x30)) as *const f32);
                        let n34 = *((cur2.wrapping_add(0x34)) as *const f32);
                        let n38 = *((cur2.wrapping_add(0x38)) as *const f32);
                        let gv4 = fsub_ss(
                            fmul_ss(fadd_ss(fadd_ss(n20, n10), n30), geom_k),
                            f40,
                        );
                        let gv2 = fsub_ss(
                            fmul_ss(fadd_ss(n34, fadd_ss(n14, n24)), geom_k),
                            f2c,
                        );
                        let gv3 = fsub_ss(
                            fmul_ss(fadd_ss(n38, fadd_ss(n18, n28)), geom_k),
                            f20,
                        );
                        // Matrix rows dotted with the geometry vector.
                        let uu = fadd_ss(
                            fadd_ss(fmul_ss(m10, gv2), fmul_ss(m00, gv4)),
                            fmul_ss(m20, gv3),
                        );
                        let vv = fadd_ss(
                            fadd_ss(fmul_ss(m14, gv2), fmul_ss(m04, gv4)),
                            fmul_ss(m24, gv3),
                        );
                        let ww = fadd_ss(
                            fadd_ss(fmul_ss(m18, gv2), fmul_ss(m08, gv4)),
                            fmul_ss(m28, gv3),
                        );
                        // Normalise scale: the compare idiom jumps unless
                        // the squared length is ordered-equal to zero,
                        // so only a zero length yields zero; anything
                        // else (NaN included) yields one over its root.
                        let n2 = fadd_ss(
                            fadd_ss(fmul_ss(vv, vv), fmul_ss(uu, uu)),
                            fmul_ss(ww, ww),
                        );
                        let root = n2.sqrt();
                        let scale = if n2 == 0.0 {
                            0.0
                        } else {
                            fdiv_ss(one, n2.sqrt())
                        };
                        let uu = fmul_ss(uu, scale);
                        let vv = fmul_ss(vv, scale);
                        let ww = fmul_ss(ww, scale);
                        // Radius gap, clamped at zero unless ordered
                        // greater (matching comiss+ja).
                        let erad = *((arg1.wrapping_add(0x20)) as *const f32);
                        let mut gap = fsub_ss(root, erad);
                        if !(gap > 0.0) {
                            gap = 0.0;
                        }
                        let no_flag =
                            *((arg0.wrapping_add(0x85)) as *const u8) == 0
                                && *((this.wrapping_add(0x15)) as *const u8)
                                    == 0;
                        if no_flag && gap > 0.0 {
                            // Positive gap with both flags clear skips
                            // the node: the scatter check below would
                            // find its stamp still zero.
                        } else {
                            let stamp = if no_flag {
                                // Non-positive gap: stamp the counter.
                                *counter
                            } else {
                                // Stepped stamp: chop the scaled gap
                                // to 64 bits (an out-of-range or NaN
                                // input stores the indefinite, whose
                                // low word is zero) and add the counter.
                                let t = fmul_ss(gap, step_k);
                                let chopped = if t.is_nan()
                                    || t >= 9.223372036854776e18f32
                                    || t <= -9.223372036854776e18f32
                                {
                                    0u32
                                } else {
                                    (t as i64) as u32
                                };
                                chopped.wrapping_add(*counter)
                            };
                            processed = processed.wrapping_add(1);
                            *((cur2.wrapping_add(0xF4)) as *mut u32) = stamp;
                            // Scaled store of the normal and the slot.
                            let e24 =
                                *((arg1.wrapping_add(0x24)) as *const f32);
                            let su = fmul_ss(uu, e24);
                            let sv = fmul_ss(vv, e24);
                            let sw = fmul_ss(ww, e24);
                            *((cur2.wrapping_add(0xDC)) as *mut f32) = slot3c;
                            *((cur2.wrapping_add(0xD8)) as *mut f32) = sw;
                            *((cur2.wrapping_add(0xD4)) as *mut f32) = sv;
                            *((cur2.wrapping_add(0xD0)) as *mut f32) = su;
                            if stamp != 0 {
                                // Scatter: three jittered components
                                // around a scaled radius.
                                let s2 = fadd_ss(
                                    fadd_ss(
                                        fmul_ss(su, su),
                                        fmul_ss(sv, sv),
                                    ),
                                    fmul_ss(sw, sw),
                                );
                                let g = fmul_ss(s2.sqrt(), scat0);
                                let neg =
                                    f32::from_bits(g.to_bits() ^ neg_mask);
                                let h = fsub_ss(g, neg);
                                let r0 = rng_draw();
                                let c0j = fadd_ss(
                                    fadd_ss(fmul_ss(fmul_ss(r0, blend_k), h), neg),
                                    su,
                                );
                                *((cur2.wrapping_add(0xD0)) as *mut f32) = c0j;
                                let r1 = rng_draw();
                                let c1j = fadd_ss(
                                    fadd_ss(fmul_ss(fmul_ss(r1, blend_k), h), neg),
                                    sv,
                                );
                                *((cur2.wrapping_add(0xD4)) as *mut f32) = c1j;
                                let r2 = rng_draw();
                                let c2j = fadd_ss(
                                    fadd_ss(fmul_ss(fmul_ss(r2, blend_k), h), neg),
                                    sw,
                                );
                                *((cur2.wrapping_add(0xD8)) as *mut f32) = c2j;
                                // Three scattered direction components.
                                let r3 = rng_draw();
                                let d0 = fsub_ss(
                                    fmul_ss(fmul_ss(r3, blend_k), scat1),
                                    one,
                                );
                                *((cur2.wrapping_add(0xE0)) as *mut f32) = d0;
                                let r4 = rng_draw();
                                let d1 = fsub_ss(
                                    fmul_ss(fmul_ss(r4, blend_k), scat1),
                                    one,
                                );
                                *((cur2.wrapping_add(0xE4)) as *mut f32) = d1;
                                let r5 = rng_draw();
                                let d2 = fsub_ss(
                                    fmul_ss(fmul_ss(r5, blend_k), scat1),
                                    one,
                                );
                                *((cur2.wrapping_add(0xE8)) as *mut f32) = d2;
                                // Normalise the scattered direction.
                                let q2 = fadd_ss(
                                    fadd_ss(
                                        fmul_ss(d0, d0),
                                        fmul_ss(d1, d1),
                                    ),
                                    fmul_ss(d2, d2),
                                );
                                let scale2 = if q2 == 0.0 {
                                    0.0
                                } else {
                                    fdiv_ss(one, q2.sqrt())
                                };
                                *((cur2.wrapping_add(0xE0)) as *mut f32) =
                                    fmul_ss(scale2, d0);
                                *((cur2.wrapping_add(0xE4)) as *mut f32) =
                                    fmul_ss(scale2, d1);
                                *((cur2.wrapping_add(0xE8)) as *mut f32) =
                                    fmul_ss(scale2, d2);
                                // One fixup call until the gate clears.
                                if *((this.wrapping_add(0x18)) as *const u32)
                                    != 0
                                {
                                    let a68 = *((arg0.wrapping_add(0x68))
                                        as *const u32);
                                    lf_checker_rt::callee_thiscall!(
                                        CAL_FIXUP,
                                        u32,
                                        lf_checker_rt::relocated(FIXUP_OBJ_VA),
                                        a68,
                                        this
                                    );
                                    *((this.wrapping_add(0x18)) as *mut u32) =
                                        0;
                                }
                            }
                        }
                    }
                    cur2 = next2;
                    if cur2 == 0 {
                        break;
                    }
                }
            }
            // Scatter tail: a fresh-enough processed list notifies
            // through the global object's vtable and stamps the age.
            if (processed as i32) > 0 {
                let tail_k =
                    *(lf_checker_rt::global::<f32>(K_TAIL_VA) as *const f32);
                let a78 = *((arg0.wrapping_add(0x78)) as *const f32);
                if tail_k > a78 {
                    let ctr_now = *counter;
                    let age = ctr_now
                        .wrapping_sub(*((this.wrapping_add(0xC4)) as *const u32));
                    if age > 0x1F4 {
                        let robj = *(lf_checker_rt::global::<u32>(VT_OBJ_VA)
                            as *const u32);
                        let carg = *((arg0.wrapping_add(0x60)) as *const u32);
                        vcall1(robj, 0x14, carg);
                        *((this.wrapping_add(0xC4)) as *mut u32) = ctr_now;
                    }
                }
            }
            // Full epilogue: copy the context tag, then check.
            let tag = *((arg1.wrapping_add(0x28)) as *const u8);
            *((arg0.wrapping_add(0x84)) as *mut u8) = tag;
            lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }
    }
});
