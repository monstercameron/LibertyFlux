// original: 0x00cc2830 ped_task_vector_update (proposed)

/// Update a task's 2-D vector and weight, then dispatch to one of two handlers.
///
/// `this` points to the task state, `arg` to a parameter block. The task
/// holds a vector (`V_X` at +0x04, `V_Y` at +0x08), per-axis limits
/// (`LIM_X` at +0x0c, `LIM_Y` at +0x10), a weight (`W` at +0x28), flag bits
/// (`FLAGS` at +0x50) and scratch fields. The parameter block holds a tag
/// (`TAG` at +0x04) and bit flags (`BITS` at +0x378).
///
/// What it does, in order: clear flag bit 5 and mark field +0x48 invalid;
/// form a scale factor F from three global floats (times a fourth when flag
/// bit 16 is set); when the squared vector length is below a global limit,
/// store a global id into +0x74; reshape each vector component against its
/// limit through float comparisons (an equality test done the original's
/// way, `ucomiss` plus a parity check, which is true only for ordered
/// equality, with a scripted unary callee consulted on the long path);
/// clamp each component into [-1, 1] once the parameter block passes its
/// bit-7/tag checks (the tag resolves through a second callee); scale the
/// weight by a global, publish the vector-plus-bias and the scaled weight
/// to globals; call handler D or E (thiscall, `this`, tag block, vector
/// address, 0, F-with-a-status-byte) selected by parameter bit 6; finally
/// set a global flag byte when task flag bit 7 is set, and return the
/// handler's byte result.
///
/// Edge cases: any float field may be NaN or infinite; every comparison is
/// an ordered SSE comparison (unordered takes the same side as the
/// original's conditional jump, including the parity-based equality tests).
/// The `FLAGS & 0x20` test can never fire (bit 5 is cleared just above) and
/// is kept literally. The status word passed to the handler is F's bits
/// with the low byte replaced by 0 or 1.
///
/// Original: 0x00cc2830 (thiscall, one stack argument, returns its result in
/// AL; 8 outgoing calls to 5 callees; reads 8 global floats/words, writes 3
/// global words and 1 global byte).
lf_checker_rt::export!(thiscall, rw_00cc2830(this: u32, arg: u32) -> u32 {
    unsafe {
        // Object layouts.
        const V_X: u32 = 0x04;
        const V_Y: u32 = 0x08;
        const LIM_X: u32 = 0x0c;
        const LIM_Y: u32 = 0x10;
        const W: u32 = 0x28;
        const W_OUT: u32 = 0x30;
        const SLOT48: u32 = 0x48;
        const FLAGS: u32 = 0x50;
        const SPEED_ID: u32 = 0x74;
        const TAG: u32 = 0x04;
        const BITS: u32 = 0x378;
        const FLAG_BIT5: u32 = 0x20;
        const FLAG_BIT16: u32 = 0x1_0000;
        const FLAG_BIT7: u32 = 0x80;
        const PARAM_BIT7: u32 = 0x80;
        const PARAM_BIT6: u32 = 0x40;
        const INVALID: u32 = 0xFFFF_FFFF;
        const NO_TAG: u32 = 0xFFFF_FFFF;
        // Globals (file VAs).
        const G_SCALE_A: u32 = 0x0117_35BC;
        const G_SCALE_B: u32 = 0x0105_1510;
        const G_SCALE_C: u32 = 0x00FE_8B08;
        const G_SPEED_LIM: u32 = 0x00FE_86B4;
        const G_SPEED_VAL: u32 = 0x0117_35B4;
        const G_BLEND_A: u32 = 0x00FE_88E8;
        const G_BLEND_B: u32 = 0x00FE_8D94;
        const G_BLEND_K: u32 = 0x0105_1514;
        const G_OUT_W: u32 = 0x0171_BFA4;
        const G_BIAS: u32 = 0x00FE_8A24;
        const G_OUT_X: u32 = 0x0171_C0D8;
        const G_OUT_Y: u32 = 0x0171_C0DC;
        const G_FLAG: u32 = 0x0171_BF96;
        // Callees.
        const CALLEE_UNARY: u32 = 1;
        const CALLEE_SYNC: u32 = 2;
        const CALLEE_RESOLVE: u32 = 3;
        const CALLEE_D: u32 = 4;
        const CALLEE_E: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        #[inline(always)]
        unsafe fn gf(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::relocated(va) as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn gd(va: u32) -> u32 {
            unsafe { (lf_checker_rt::relocated(va) as *const u32).read_unaligned() }
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

        let blend_a = gf(G_BLEND_A);
        let blend_b = gf(G_BLEND_B);

        // Scale factor F.
        let mut f = mul(gf(G_SCALE_B), gf(G_SCALE_A));
        let mut flags = rd32(this + FLAGS) & !FLAG_BIT5;
        wr32(this + FLAGS, flags);
        flags = rd32(this + FLAGS);
        wr32(this + SLOT48, INVALID);
        let mut fslot = f.to_bits();
        if flags & FLAG_BIT16 != 0 {
            f = mul(f, gf(G_SCALE_C));
            fslot = f.to_bits();
        }
        // Dead in the original too: bit 5 was just cleared above.
        if flags & FLAG_BIT5 != 0 {
            f = 0.0;
            fslot = 0;
        }

        // Squared-length check.
        let vx = rdf(this + V_X);
        let vy = rdf(this + V_Y);
        let len2 = add(mul(vx, vx), mul(vy, vy));
        if !(gf(G_SPEED_LIM) <= len2) {
            wr32(this + SPEED_ID, gd(G_SPEED_VAL));
        }

        // Reshape V_X against LIM_X.
        let mut cx = rdf(this + V_X);
        let limx = rdf(this + LIM_X);
        if limx <= cx {
            // Long path: consult the unary callee unless the value is zero.
            if !(cx <= limx) {
                if cx == 0.0 {
                    cx = sub(cx, f);
                } else {
                    let t1: f32 = lf_checker_rt::callee_cdecl!(CALLEE_UNARY, f32, limx.to_bits());
                    let t2: f32 = lf_checker_rt::callee_cdecl!(CALLEE_UNARY, f32, cx.to_bits());
                    if t1 == t2 {
                        cx = sub(cx, f);
                    } else {
                        cx = sub(cx, mul(gf(G_BLEND_K), f));
                    }
                }
                wrf(this + V_X, cx);
                if !(limx <= cx) {
                    wrf(this + V_X, limx);
                }
            } else {
                wrf(this + V_X, limx);
            }
        } else {
            // Short path: blend picks, then an equality test.
            if cx == 0.0 {
                wrf(this + V_X, add(cx, f));
            } else {
                let x0 = if 0.0 <= limx {
                    if limx == 0.0 { 0.0 } else { blend_a }
                } else {
                    blend_b
                };
                let x3 = if 0.0 <= cx { blend_a } else { blend_b };
                if x0 == x3 {
                    wrf(this + V_X, add(cx, f));
                } else {
                    wrf(this + V_X, add(mul(gf(G_BLEND_K), f), cx));
                }
            }
            let cur = rdf(this + V_X);
            if !(cur <= limx) {
                wrf(this + V_X, limx);
            }
        }

        // Reshape V_Y against LIM_Y (mirror image of the V_X block).
        let mut cy = rdf(this + V_Y);
        let limy = rdf(this + LIM_Y);
        if limy <= cy {
            if !(cy <= limy) {
                if cy == 0.0 {
                    cy = sub(cy, f);
                } else {
                    let t1: f32 = lf_checker_rt::callee_cdecl!(CALLEE_UNARY, f32, limy.to_bits());
                    let t2: f32 = lf_checker_rt::callee_cdecl!(CALLEE_UNARY, f32, cy.to_bits());
                    if t1 == t2 {
                        cy = sub(cy, f);
                    } else {
                        cy = sub(cy, mul(gf(G_BLEND_K), f));
                    }
                }
                wrf(this + V_Y, cy);
                if !(limy <= cy) {
                    wrf(this + V_Y, limy);
                }
            } else {
                wrf(this + V_Y, limy);
            }
        } else if cy == 0.0 {
            wrf(this + V_Y, add(cy, f));
            let cur = rdf(this + V_Y);
            if !(cur <= limy) {
                wrf(this + V_Y, limy);
            }
        } else {
            let x4 = if 0.0 <= limy {
                if limy == 0.0 { 0.0 } else { blend_a }
            } else {
                blend_b
            };
            let x0 = if 0.0 <= cy { blend_a } else { blend_b };
            if x4 == x0 {
                wrf(this + V_Y, add(cy, f));
            } else {
                wrf(this + V_Y, add(mul(gf(G_BLEND_K), f), cy));
            }
            let cur = rdf(this + V_Y);
            if !(cur <= limy) {
                wrf(this + V_Y, limy);
            }
        }

        // Publish the scaled weight.
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SYNC, u32, this);
        let w = mul(rdf(this + W), gf(G_SCALE_A));
        fslot = (fslot & !0xFF) | 0x00;
        wrf(lf_checker_rt::relocated(G_OUT_W), w);
        wrf(this + W_OUT, w);

        // Parameter checks, then clamp the vector into [-1, 1].
        let mut tail = false;
        if rd32(arg + BITS) & PARAM_BIT7 != 0 {
            tail = true;
        } else {
            let tag = rd32(arg + TAG);
            if tag != NO_TAG {
                let p: u32 = lf_checker_rt::callee_cdecl!(CALLEE_RESOLVE, u32, tag);
                if rd32(p + BITS) & PARAM_BIT7 != 0 {
                    tail = true;
                }
            }
        }
        if !tail {
            fslot = (fslot & !0xFF) | 0x01;
            let qx = rdf(this + V_X);
            if blend_b <= qx {
                if !(qx <= blend_a) {
                    wrf(this + V_X, 1.0);
                }
            } else {
                wrf(this + V_X, -1.0);
            }
            let qy = rdf(this + V_Y);
            if blend_b <= qy {
                if !(qy <= blend_a) {
                    wrf(this + V_Y, 1.0);
                }
            } else {
                wrf(this + V_Y, -1.0);
            }
        }

        // Publish the biased vector and dispatch to handler D or E.
        wrf(lf_checker_rt::relocated(G_OUT_X), add(rdf(this + V_X), gf(G_BIAS)));
        wrf(lf_checker_rt::relocated(G_OUT_Y), add(rdf(this + V_Y), gf(G_BIAS)));
        let ret: u32 = if rd32(arg + BITS) & PARAM_BIT6 != 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_D, u32, this, arg, this + V_X, 0, fslot)
        } else {
            lf_checker_rt::callee_thiscall!(CALLEE_E, u32, this, arg, this + V_X, 0, fslot)
        };

        // Global flag byte, then return the handler's byte.
        let s50 = rd32(this + FLAGS);
        let gb = (lf_checker_rt::relocated(G_FLAG) as *const u8).read_unaligned();
        let newgb = if s50 & FLAG_BIT7 != 0 { 1 } else { gb };
        (lf_checker_rt::relocated(G_FLAG) as *mut u8).write_unaligned(newgb);
        ret & 0xFF
    }
});
