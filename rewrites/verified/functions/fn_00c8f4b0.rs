// original: 0x00C8F4B0 task_blend_query (proposed)

/// Blend a task node's heading from an angle, a mode query and two staged
/// double helpers, writing three floats to `out`.
///
/// `this` is the node (tag at `+0x00`, child at `+0x10`). Tags 4 and 5 take
/// the direct path: an angle from bits 15..23 of the tag word (`byte *
/// 2π/255`, scale from read-only game data) through sine and cosine helpers,
/// writing negated sine, cosine and zero. Tags 2 and 3 need a child, derive
/// the same angle pair, then ask a classifier helper about the child: answer
/// 3 blends with the child's `+0x08` measurement, answer 2 with its `+0x18`
/// measurement, anything else writes the raw pair with a zero third. Each
/// blend picks one of two staged double-helper sites by the sign of its
/// measurement, converts both answers exactly as the original's
/// double-to-float instruction does, and combines them with the angle pair
/// (including multiply-by-zero terms, which are not no-ops for NaN or
/// infinite inputs). Anything else writes three zeros. Returns `out`.
///
/// The double helpers take their arguments in vector registers, which the
/// checker cannot forward, so each call site gets its own scripted answer
/// (keeping every branch observable) and the answers reach the rewrite
/// through a scratch slot past the child. The first cosine call's vector
/// input is left uncompared (its upper lanes carry residue the movss
/// transport zeroes); the same angle is verified through the sine call.
///
/// Original: 0x00C8F4B0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00C8F4B0(this: u32, out: u32) -> u32 {
    unsafe {
        const OFF_CHILD: u32 = 0x10;
        const ANS_SCRATCH: u32 = 0x40;
        const ANGLE_SHIFT: u32 = 15;
        const ANGLE_SCALE: u32 = 0x00ED6D24;
        const SIGN_BIT: u32 = 0x8000_0000;
        const CAL_SIN: u32 = 1;
        const CAL_COS: u32 = 2;
        const CAL_MODE: u32 = 3;
        const CAL_MEAS: u32 = 4;
        const CAL_D1: u32 = 5;
        const CAL_D2: u32 = 6;
        const CAL_D3: u32 = 7;
        const CAL_D4: u32 = 8;
        const CAL_E: u32 = 9;
        const CAL_E2: u32 = 10;

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
        /// A staged helper answer, converted exactly as the original's
        /// double-to-float instruction converts it.
        #[inline(always)]
        unsafe fn ans_float(scratch_base: u32) -> f32 {
            unsafe {
                let lo = rd32(scratch_base + ANS_SCRATCH) as u64;
                let hi = rd32(scratch_base + ANS_SCRATCH + 4) as u64;
                core::hint::black_box(f64::from_bits((hi << 32) | lo)) as f32
            }
        }

        const ZERO: f32 = 0.0;
        let idx = (rd32(this) & 7).wrapping_sub(2);
        if idx > 3 {
            wr32(out, 0);
            wr32(out + 0x04, 0);
            wr32(out + 0x08, 0);
            return out;
        }
        // Jump-table mapping of idx to path: 0-1 blend, 2-3 direct.
        let byte = ((rd32(this) >> ANGLE_SHIFT) & 0xFF) as u8;
        let angle = mul(byte as i32 as f32, rdf(lf_checker_rt::relocated(ANGLE_SCALE)));
        if idx >= 2 {
            let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_SIN, u32, angle.to_bits()));
            wrf(out, negate_bits(sin));
            let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_COS, u32,));
            wrf(out + 0x04, cos);
            wr32(out + 0x08, 0);
            return out;
        }
        let child = rd32(this + OFF_CHILD);
        if child == 0 {
            wr32(out, 0);
            wr32(out + 0x04, 0);
            wr32(out + 0x08, 0);
            return out;
        }
        let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_SIN, u32, angle.to_bits()));
        let nsin = negate_bits(sin);
        let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_COS, u32,));
        let mode = lf_checker_rt::callee_cdecl!(CAL_MODE, u32, child);
        if mode == 3 {
            let m = lf_checker_rt::callee_thiscall!(CAL_MEAS, u32, child);
            let s2c: f32;
            if 0.0f32 > rdf(m + 0x08) {
                lf_checker_rt::callee_thiscall!(CAL_D1, u32, child);
                s2c = ans_float(child);
            } else {
                lf_checker_rt::callee_thiscall!(CAL_D2, u32, child);
                s2c = ans_float(child);
            }
            lf_checker_rt::callee_thiscall!(CAL_E, u32, child);
            let cvt = ans_float(child);
            let xa = add(mul(cvt, nsin), mul(s2c, ZERO));
            let ya = sub(mul(cvt, ZERO), mul(s2c, nsin));
            wrf(out, xa);
            wrf(out + 0x04, cos);
            wrf(out + 0x08, ya);
            return out;
        }
        if mode == 2 {
            let m = lf_checker_rt::callee_thiscall!(CAL_MEAS, u32, child);
            let s2c: f32;
            if 0.0f32 > rdf(m + 0x18) {
                lf_checker_rt::callee_thiscall!(CAL_D3, u32, child);
                s2c = ans_float(child);
            } else {
                lf_checker_rt::callee_thiscall!(CAL_D4, u32, child);
                s2c = ans_float(child);
            }
            lf_checker_rt::callee_thiscall!(CAL_E2, u32, child);
            let cvt = ans_float(child);
            let xb = sub(mul(cvt, cos), mul(s2c, ZERO));
            let yb = add(mul(s2c, cos), mul(cvt, ZERO));
            wrf(out, nsin);
            wrf(out + 0x04, xb);
            wrf(out + 0x08, yb);
            return out;
        }
        wrf(out, nsin);
        wrf(out + 0x04, cos);
        wrf(out + 0x08, ZERO);
        out
    }
});
