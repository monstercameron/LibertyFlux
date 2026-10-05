// original: 0x00b4ec40 ped_task_proximity_check (proposed)

/// Decide whether `this` is the context's near target, then report it.
///
/// `this` and the context found through a parameterless call each carry a
/// position vector at `+0x20` (`x,y,z` floats at `+0x30/34/38`). A chain of
/// gates must all pass, else 0 is returned: the context exists; a sub
/// object opens twice; the resolved info word has bit 5 set; a candidate
/// (the context's word at `+0x398`, or a queried slot object kept unless a
/// `0.2` threshold exceeds the query's out-word) equals `this`; the
/// squared distance is strictly below the squared range global; and the
/// frame's second-row dot with the delta is strictly negative.
///
/// Past the gates, two more frame dots (`v` from the first row, `w` from
/// the second) are negated by sign-bit xor, folded to `|w|`,`|v|` via
/// `(x > 0) ? x : -x` (exact NaN and signed-zero behaviour kept), and
/// compared: `|w| > |v|` yields code `0x131 + 2*(w > 0)`, else
/// `0x12d + 2*!(v > 0)`. A hub object from a global then emits the code
/// with weight `16.0` and tag `0x42` (skipped, handle 0, when the hub
/// lookup fails), optionally notifies with the context, attaches the
/// notify result to the handle, and commits with `this`. Returns 1. All
/// float arithmetic runs in the original's operand order through
/// order-pinning helpers. Thiscall, no stack words, byte result.
lf_checker_rt::export!(thiscall, rw_00b4ec40(this: u32) -> u32 {
    unsafe {
        const CTX_VEC: u32 = 0x20;
        const V_X: u32 = 0x30;
        const V_Y: u32 = 0x34;
        const V_Z: u32 = 0x38;
        const SUB_OFF: u32 = 0x2B0;
        const INFO_WORD: u32 = 0x20;
        const CAND: u32 = 0x398;
        const SLOT_BASE: u32 = 0x224;
        const SLOT_BIAS: u32 = 0x44;
        const SLOT_ARG: u32 = 6;
        const INFO2: u32 = 0x18;
        const M1_R1: u32 = 0x10;
        const M1_R2: u32 = 0x14;
        const M1_R3: u32 = 0x18;
        const M2_R1: u32 = 0x00;
        const M2_R2: u32 = 0x04;
        const M2_R3: u32 = 0x08;
        const C_THRESH: u32 = 0xFE87D0;
        const C_RANGE: u32 = 0x1046064;
        const C_SIGN: u32 = 0xFE8FA0;
        const G_HUB: u32 = 0x171FAF4;
        const CODE_A: u32 = 0x131;
        const CODE_B: u32 = 0x12D;
        const WEIGHT: u32 = 0x41800000;
        const TAG: u32 = 0x42;
        const FIND_CTX: u32 = 1;
        const OPEN_SUB: u32 = 2;
        const RESOLVE: u32 = 3;
        const FIND_SLOT: u32 = 4;
        const QUERY: u32 = 5;
        const FRAME: u32 = 6;
        const HUB: u32 = 7;
        const EMIT: u32 = 8;
        const NOTIFY: u32 = 9;
        const ATTACH: u32 = 10;
        const COMMIT: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn neg_bits(x: f32, sign: u32) -> f32 {
            f32::from_bits(x.to_bits() ^ sign)
        }

        let ctx: u32 = lf_checker_rt::callee_thiscall!(FIND_CTX, u32, this);
        if ctx == 0 {
            return 0;
        }
        let subobj = ctx.wrapping_add(SUB_OFF);
        let g1: u32 = lf_checker_rt::callee_thiscall!(OPEN_SUB, u32, subobj);
        if g1 == 0 {
            return 0;
        }
        let g2: u32 = lf_checker_rt::callee_thiscall!(OPEN_SUB, u32, subobj);
        let info: u32 = lf_checker_rt::callee_cdecl!(RESOLVE, u32, rd32(g2 + INFO2));
        if (rd32(info + INFO_WORD) >> 5) & 1 == 0 {
            return 0;
        }
        let mut cand = rd32(ctx + CAND);
        if cand == 0 {
            let slot: u32 = lf_checker_rt::callee_thiscall!(
                FIND_SLOT, u32,
                rd32(ctx + SLOT_BASE).wrapping_add(SLOT_BIAS),
                SLOT_ARG
            );
            if slot != 0 {
                let mut out = 0u32;
                let r: u32 = lf_checker_rt::callee_thiscall!(
                    QUERY, u32, slot, &mut out as *mut u32 as u32, 0
                );
                let thresh = rdf(lf_checker_rt::relocated(C_THRESH));
                if thresh > f32::from_bits(out) {
                    cand = 0;
                } else {
                    cand = r;
                }
            }
        }
        if cand != this {
            return 0;
        }
        let v0 = rd32(this + CTX_VEC);
        let v1 = rd32(ctx + CTX_VEC);
        let dx = sub(rdf(v0 + V_X), rdf(v1 + V_X));
        let dy = sub(rdf(v0 + V_Y), rdf(v1 + V_Y));
        let dz = sub(rdf(v0 + V_Z), rdf(v1 + V_Z));
        let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let range = rdf(lf_checker_rt::relocated(C_RANGE));
        if !(mul(range, range) > dist2) {
            return 0;
        }
        let m1: u32 = lf_checker_rt::callee_thiscall!(FRAME, u32, this);
        let dot1 = add(
            add(mul(rdf(m1 + M1_R2), dy), mul(rdf(m1 + M1_R1), dx)),
            mul(rdf(m1 + M1_R3), dz),
        );
        if !(dot1 < 0.0) {
            return 0;
        }
        let m2: u32 = lf_checker_rt::callee_thiscall!(FRAME, u32, this);
        let vv = add(
            add(mul(rdf(m2 + M2_R2), dy), mul(rdf(m2 + M2_R1), dx)),
            mul(rdf(m2 + M2_R3), dz),
        );
        let sign = rd32(lf_checker_rt::relocated(C_SIGN));
        let nv = neg_bits(vv, sign);
        let m3: u32 = lf_checker_rt::callee_thiscall!(FRAME, u32, this);
        let ww = add(
            add(mul(rdf(m3 + M1_R2), dy), mul(rdf(m3 + M1_R1), dx)),
            mul(rdf(m3 + M1_R3), dz),
        );
        let nw = neg_bits(ww, sign);
        let x3 = if ww > 0.0 { ww } else { nw };
        let x2 = if vv > 0.0 { vv } else { nv };
        let code: u32 = if x3 > x2 {
            (if ww > 0.0 { 1u32 } else { 0u32 })
                .wrapping_mul(2)
                .wrapping_add(CODE_A)
        } else {
            (if vv > 0.0 { 0u32 } else { 1u32 })
                .wrapping_mul(2)
                .wrapping_add(CODE_B)
        };
        let hub = rd32(lf_checker_rt::relocated(G_HUB));
        let h1: u32 = lf_checker_rt::callee_thiscall!(HUB, u32, hub);
        let dev: u32 = if h1 == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(EMIT, u32, h1, TAG, code, WEIGHT)
        };
        let h2: u32 = lf_checker_rt::callee_thiscall!(HUB, u32, hub);
        let rep: u32 = if h2 == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(NOTIFY, u32, h2, ctx, 1, 0, 0)
        };
        lf_checker_rt::callee_thiscall!(ATTACH, u32, dev, rep);
        lf_checker_rt::callee_thiscall!(COMMIT, u32, this, dev, 1);
        1
    }
});
