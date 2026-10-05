// original: 0x00cf7b80 ladder_mount_approach_solve (proposed)

/// Solve the ladder-mount approach: facing angle plus approach points.
///
/// `obj_a` carries the target matrix (translation at `+0x30` of the object
/// at its `+0x20`, always present); `obj_b` selects a task node by its
/// signed type id at `+0x2E` and `index`, exactly like the sibling
/// function, and holds the lazily built matrix at `+0x20`. `out0` and
/// `out1` receive two transformed coordinates each, `out2` a full float4
/// approach point, `float_out` the facing angle, and `work` is a scratch
/// vector that is fully overwritten. `mode` picks the approach side and
/// `flag9` (low byte) enables a facing dot-product check.
///
/// Behaviour: look the node up and require kind `0x0E` (else return 0
/// with the kind's high bytes kept), fetch the anchor source, seed the
/// work vector from its third anchor, and build the matrix when missing.
/// Transform the first two anchors by the matrix into `xa`/`xb` pairs,
/// transform the work vector the same way (no translation) and take its
/// facing angle through the four-float arctangent helper, storing it to
/// `float_out`. Measure the target translation against the second pair and
/// scale by the reciprocal root (an exactly zero distance scales by 0
/// instead). With the check flag
/// set, dot the scaled offset with the work vector and return 0 with the
/// target-row high bytes (second byte replaced by the distance-compare
/// flags via lahf) unless the dot is positive (mode 1) or negative
/// (other modes); a NaN dot continues on both. Otherwise write the two
/// pairs, then the approach point: scaled work vector added to the
/// second pair (mode 1) or subtracted from the first pair, with the third
/// component read back from the half-written outputs. The fourth
/// components of `work` and `out2` come from two frame dwords nothing
/// ever writes (uninitialized stack, proven by slot simulation); under
/// the checker's defined zero stack fill they are 0, so this rewrite
/// stores `0.0`, and in the game they are indeterminate. Returns
/// `(out2 & ~0xFF) | 1`.
///
/// Original: 0x00CF7B80 (cdecl/10; reads no registers on entry).
lf_checker_rt::export!(cdecl, rw_00cf7b80(obj_a: u32, obj_b: u32, index: u32, out0: u32, out1: u32, out2: u32, float_out: u32, mode: u32, work: u32, flag9: u32) -> u32 {
    unsafe {
        const TYPE_ID: u32 = 0x2E;
        const MATRIX_PTR: u32 = 0x20;
        const POS_HEADING: u32 = 0x10;
        const TYPE_TABLE: u32 = 0x1295CD8;
        const KIND_OK: u32 = 0x0E;
        const VF_KIND: u32 = 4;
        const VF_SRC: u32 = 0x44;
        const GAIN: u32 = 0x10539EC;
        const UNIT: u32 = 0xFE88E8;
        const CAL_LOOKUP: u32 = 0;
        const CAL_ALLOC: u32 = 3;
        const CAL_BUILD: u32 = 4;
        const CAL_ATAN: u32 = 5;
        /// Value of the two uninitialized frame slots under `stack_fill: 0`
        /// (see the doc comment: the original reads them, nothing writes).
        const UNINIT_W: f32 = 0.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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

        let type_id = ((obj_b.wrapping_add(TYPE_ID)) as *const i16).read_unaligned() as i32;
        let entry = (lf_checker_rt::relocated(TYPE_TABLE)
            .wrapping_add((type_id as u32).wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let node: u32 = lf_checker_rt::callee_thiscall!(CAL_LOOKUP, u32, entry, index);
        let vtbl = (node as *const u32).read_unaligned();
        let f_kind: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtbl.wrapping_add(VF_KIND)) as usize);
        let kind = f_kind(node);
        if kind & 0xFF != KIND_OK {
            return kind & 0xFFFFFF00;
        }
        let f_src: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtbl.wrapping_add(VF_SRC)) as usize);
        let src = f_src(node);
        // Anchor components (kept in locals, as the original's spills do).
        let s10 = rdf(src.wrapping_add(0x10));
        let s14 = rdf(src.wrapping_add(0x14));
        let s18 = rdf(src.wrapping_add(0x18));
        let s1c = rdf(src.wrapping_add(0x1C));
        let s20 = rdf(src.wrapping_add(0x20));
        let s24 = rdf(src.wrapping_add(0x24));
        // Seed the work vector from the third anchor.
        wrf(work, rdf(src.wrapping_add(0x28)));
        wrf(work.wrapping_add(4), rdf(src.wrapping_add(0x2C)));
        wrf(work.wrapping_add(8), rdf(src.wrapping_add(0x30)));
        if rd32(obj_b.wrapping_add(MATRIX_PTR)) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(CAL_ALLOC, u32, obj_b);
            let m_now = rd32(obj_b.wrapping_add(MATRIX_PTR));
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CAL_BUILD,
                u32,
                obj_b.wrapping_add(POS_HEADING),
                m_now
            );
        }
        let m = rd32(obj_b.wrapping_add(MATRIX_PTR));
        let m0 = rdf(m);
        let m1 = rdf(m.wrapping_add(4));
        let m4 = rdf(m.wrapping_add(0x10));
        let m5 = rdf(m.wrapping_add(0x14));
        let m8 = rdf(m.wrapping_add(0x20));
        let m9 = rdf(m.wrapping_add(0x24));
        let t0 = rdf(m.wrapping_add(0x30));
        let t1 = rdf(m.wrapping_add(0x34));
        // First anchor pair, translated.
        let mut xa0 = add(mul(m0, s1c), mul(m4, s20));
        xa0 = add(xa0, mul(m8, s24));
        xa0 = add(xa0, t0);
        let mut xa1 = add(mul(m1, s1c), mul(m5, s20));
        xa1 = add(xa1, mul(m9, s24));
        xa1 = add(xa1, t1);
        // Second anchor pair, translated.
        let mut xb0 = add(mul(m0, s10), mul(m4, s14));
        xb0 = add(xb0, mul(m8, s18));
        xb0 = add(xb0, t0);
        let mut xb1 = add(mul(m1, s10), mul(m5, s14));
        xb1 = add(xb1, mul(m9, s18));
        xb1 = add(xb1, t1);
        // Work vector transform (no translation), in the original's order:
        // xw = (m4*w1 + m0*w0) + m8*w2, yw = (m1*w0 + m5*w1) + m9*w2.
        // (Matrix rows are the transposed loads the original issues.)
        let w0 = rdf(work);
        let w1 = rdf(work.wrapping_add(4));
        let w2 = rdf(work.wrapping_add(8));
        let mut xw = add(mul(m4, w1), mul(m0, w0));
        xw = add(xw, mul(m8, w2));
        let mut yw = add(mul(m1, w0), mul(m5, w1));
        yw = add(yw, mul(m9, w2));
        wrf(work.wrapping_add(0x0C), UNINIT_W);
        wrf(work.wrapping_add(8), 0.0);
        wrf(work.wrapping_add(4), yw);
        wrf(work, xw);
        let atan: f32 = lf_checker_rt::callee_cdecl!(CAL_ATAN, f32, 0, 0, xw.to_bits(), yw.to_bits());
        wrf(float_out, atan);
        // Offset of the target translation from the second pair.
        let m2 = rd32(obj_a.wrapping_add(MATRIX_PTR));
        let dx = sub(rdf(m2.wrapping_add(0x30)), xb0);
        let dy = sub(rdf(m2.wrapping_add(0x34)), xb1);
        let dist2 = add(mul(dy, dy), mul(dx, dx));
        // The scale is the reciprocal root, except for an exactly zero
        // distance, which scales by 0: the jp tests the parity of (lahf &
        // 0x44), which is odd only for the equal case (0x42 & 0x44 = 0x40),
        // so every other case (positive, infinite, NaN) takes the root.
        let unit = rdf(lf_checker_rt::relocated(UNIT));
        let k = if dist2 == 0.0 {
            0.0
        } else {
            core::hint::black_box(unit) / core::hint::black_box(dist2.sqrt())
        };
        let dxk = mul(dx, k);
        let dyk = mul(dy, k);
        let k0 = mul(k, 0.0);
        // The distance compare loads the flags into AH (lahf) right after,
        // and that byte survives in eax to the dot-check early return:
        // 0x02 when the distance is positive, 0x42 when it is +0, 0x47
        // when it is NaN (ucomiss clears SF/AF; a sum of squares is never
        // negative). The low byte is then cleared by (an instruction of the original).
        let lahf_ah: u32 = if dist2.is_nan() {
            0x47
        } else if dist2 == 0.0 {
            0x42
        } else {
            0x02
        };
        let dot_early =
            ((m2.wrapping_add(0x30) & 0xFFFF00FF) | (lahf_ah << 8)) & 0xFFFFFF00;
        if flag9 & 0xFF != 0 {
            let cw0 = rdf(work);
            let cw1 = rdf(work.wrapping_add(4));
            let cw2 = rdf(work.wrapping_add(8));
            let mut dot = add(mul(dxk, cw0), mul(dyk, cw1));
            dot = add(dot, mul(cw2, k0));
            if mode == 1 {
                if dot <= 0.0 {
                    return dot_early;
                }
            } else if dot >= 0.0 {
                return dot_early;
            }
        }
        wrf(out0, xa0);
        wrf(out0.wrapping_add(4), xa1);
        wrf(out1, xb0);
        wrf(out1.wrapping_add(4), xb1);
        let gain = rdf(lf_checker_rt::relocated(GAIN));
        let wv0 = rdf(work);
        let wv1 = rdf(work.wrapping_add(4));
        let wv2 = rdf(work.wrapping_add(8));
        let t = mul(wv0, gain);
        if mode == 1 {
            wrf(out2, add(t, xb0));
            wrf(out2.wrapping_add(4), add(mul(gain, wv1), xb1));
            wrf(out2.wrapping_add(8), add(rdf(out1.wrapping_add(8)), mul(wv2, gain)));
        } else {
            wrf(out2, sub(xa0, t));
            wrf(out2.wrapping_add(4), sub(xa1, mul(gain, wv1)));
            wrf(out2.wrapping_add(8), sub(rdf(out0.wrapping_add(8)), mul(wv2, gain)));
        }
        wrf(out2.wrapping_add(0x0C), UNINIT_W);
        (out2 & 0xFFFFFF00) | 1
    }
});
