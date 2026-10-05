// original: 0x00C8EF60 task_solve_query (proposed)

/// Solve one task node into a four-float buffer, reporting success in `al`.
///
/// `this` is the node (tag at `+0x00`, position at `+0x04..+0x0c`, child at
/// `+0x10`), `out` the buffer, `aux` an optional second input. The tag's low
/// 3 bits select the path: tag 1 transforms the node's position by the
/// child's 3x4 matrix (creating the matrix on demand); tags 2 and 3 run the
/// full solver (an arctangent branch from the child's matrix exactly like the
/// heading query, a sine/cosine rotation of the position, an optional second
/// matrix accumulation from `aux`, and a translation add); tags 4 and 5 copy
/// the position and add `aux` with a per-lane operand order. Anything else, a
/// null child, or (on the copy path) a null `aux` still succeeding, returns
/// 1; the default and null-child paths return 0. The fourth output word is
/// whatever uninitialized stack residue the original reads; the contract pins
/// that residue to zero.
///
/// The arctangent helpers take doubles in vector registers, which the checker
/// cannot forward, so each call site gets its own scripted answer (keeping
/// every branch observable) and the answers reach the rewrite through a
/// scratch slot past the child. The sine call's vector input is left
/// uncompared (its upper lane carries residue of the scripted double); the
/// same converted value is verified through the logged cosine call.
///
/// Original: 0x00C8EF60 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00C8EF60(this: u32, out: u32, aux: u32) -> u32 {
    unsafe {
        const OFF_CHILD: u32 = 0x10;
        const OFF_MAT: u32 = 0x20;
        const OFF_FALLBACK: u32 = 0x1c;
        const ANS_SCRATCH: u32 = 0x40;
        const THRESH_BITS: u32 = 0x3F666666; // 0.9, the axis-dominance cutoff
        const SIGN_BIT: u32 = 0x8000_0000;
        const CAL_SIN: u32 = 1;
        const CAL_COS: u32 = 2;
        const CAL_ATAN_A: u32 = 3;
        const CAL_ATAN_B: u32 = 4;
        const CAL_MAT2: u32 = 5;
        const CAL_INIT: u32 = 6;
        const CAL_LINK: u32 = 7;

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
        #[inline(always)]
        fn negate_bits(v: f32) -> f32 {
            f32::from_bits(v.to_bits() ^ SIGN_BIT)
        }
        /// Absolute value exactly as the original's compare-and-xor sequence.
        #[inline(always)]
        fn abs_select(v: f32) -> f32 {
            if 0.0f32 > v {
                negate_bits(v)
            } else {
                v
            }
        }
        /// A scripted arctangent answer, converted exactly as the original's
        /// double-to-float instruction converts it.
        #[inline(always)]
        unsafe fn atan_float(scratch_base: u32) -> f32 {
            unsafe {
                let lo = rd32(scratch_base + ANS_SCRATCH) as u64;
                let hi = rd32(scratch_base + ANS_SCRATCH + 4) as u64;
                core::hint::black_box(f64::from_bits((hi << 32) | lo)) as f32
            }
        }
        /// Ensure the child's matrix exists, creating it on demand.
        #[inline(always)]
        unsafe fn ensure_mat(child: u32) -> u32 {
            unsafe {
                if rd32(child + OFF_MAT) == 0 {
                    lf_checker_rt::callee_thiscall!(CAL_INIT, u32, child);
                    let fresh = rd32(child + OFF_MAT);
                    lf_checker_rt::callee_thiscall!(CAL_LINK, u32, child.wrapping_add(OFF_CHILD), fresh);
                }
                rd32(child + OFF_MAT)
            }
        }

        let idx = (rd32(this) & 7).wrapping_sub(1);
        if idx > 4 {
            return 0;
        }
        // Jump-table mapping of idx to path: 0 transform, 1-2 solve, 3-4 copy.
        if idx >= 3 {
            wr32(out, rd32(this + 0x04));
            wr32(out + 0x04, rd32(this + 0x08));
            wr32(out + 0x08, rd32(this + 0x0c));
            if aux != 0 {
                wrf(out, add(rdf(out), rdf(aux)));
                wrf(out + 0x04, add(rdf(aux + 0x04), rdf(out + 0x04)));
                wrf(out + 0x08, add(rdf(aux + 0x08), rdf(out + 0x08)));
            }
            return 1;
        }
        if idx == 0 {
            let child = rd32(this + OFF_CHILD);
            if child == 0 {
                return 0;
            }
            let v0 = rdf(this + 0x04);
            let v1 = rdf(this + 0x08);
            let v2 = rdf(this + 0x0c);
            let mat = ensure_mat(child);
            let r0 = add(add(add(mul(rdf(mat + 0x10), v1), mul(rdf(mat), v0)), mul(rdf(mat + 0x20), v2)), rdf(mat + 0x30));
            let r1 = add(add(add(mul(rdf(mat + 0x14), v1), mul(rdf(mat + 0x04), v0)), mul(rdf(mat + 0x24), v2)), rdf(mat + 0x34));
            let r2 = add(add(add(mul(rdf(mat + 0x18), v1), mul(rdf(mat + 0x08), v0)), mul(rdf(mat + 0x28), v2)), rdf(mat + 0x38));
            wrf(out, r0);
            wrf(out + 0x04, r1);
            wrf(out + 0x08, r2);
            wr32(out + 0x0c, 0);
            return 1;
        }
        let child = rd32(this + OFF_CHILD);
        if child == 0 {
            return 0;
        }
        let v0 = rdf(this + 0x04);
        let v1 = rdf(this + 0x08);
        let v2 = rdf(this + 0x0c);
        wrf(out, v0);
        wrf(out + 0x04, v1);
        wrf(out + 0x08, v2);
        wr32(out + 0x0c, 0);
        let mat = rd32(child + OFF_MAT);
        let thresh = f32::from_bits(THRESH_BITS);
        let a: f32;
        if abs_select(rdf(mat + 0x28)) > thresh {
            if mat == 0 {
                a = rdf(child + OFF_FALLBACK);
            } else {
                lf_checker_rt::callee_thiscall!(CAL_ATAN_B, u32, child);
                a = atan_float(child);
            }
        } else if abs_select(rdf(mat + 0x18)) > thresh {
            lf_checker_rt::callee_thiscall!(CAL_ATAN_A, u32, child);
            a = atan_float(child);
        } else if abs_select(rdf(mat + 0x08)) > thresh {
            lf_checker_rt::callee_thiscall!(CAL_ATAN_A, u32, child);
            a = atan_float(child);
        } else if mat == 0 {
            a = rdf(child + OFF_FALLBACK);
        } else {
            lf_checker_rt::callee_thiscall!(CAL_ATAN_B, u32, child);
            a = atan_float(child);
        }
        let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_SIN, u32,));
        let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_COS, u32, a.to_bits()));
        wrf(out, sub(mul(v0, cos), mul(v1, sin)));
        wrf(out + 0x04, add(mul(v1, cos), mul(v0, sin)));
        if aux != 0 {
            let u0 = rdf(aux);
            let u1 = rdf(aux + 0x04);
            let u2 = rdf(aux + 0x08);
            let m2 = lf_checker_rt::callee_thiscall!(CAL_MAT2, u32, child);
            let t0 = add(add(mul(rdf(m2 + 0x10), u1), mul(rdf(m2), u0)), mul(rdf(m2 + 0x20), u2));
            let t1 = add(add(mul(rdf(m2 + 0x14), u1), mul(rdf(m2 + 0x04), u0)), mul(rdf(m2 + 0x24), u2));
            let t2 = add(add(mul(rdf(m2 + 0x18), u1), mul(rdf(m2 + 0x08), u0)), mul(rdf(m2 + 0x28), u2));
            wrf(out, add(rdf(out), t0));
            wrf(out + 0x04, add(rdf(out + 0x04), t1));
            wrf(out + 0x08, add(rdf(out + 0x08), t2));
        }
        let mat2 = ensure_mat(child);
        wrf(out, add(rdf(mat2 + 0x30), rdf(out)));
        wrf(out + 0x04, add(rdf(mat2 + 0x34), rdf(out + 0x04)));
        wrf(out + 0x08, add(rdf(mat2 + 0x38), rdf(out + 0x08)));
        1
    }
});
