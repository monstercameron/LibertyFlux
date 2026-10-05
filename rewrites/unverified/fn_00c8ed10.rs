// original: 0x00C8ED10 task_heading_query (proposed)

/// Query a heading vector for one task node into a three-float buffer.
///
/// `this` is the node (tag at `+0x00`, child at `+0x10`), `out` the buffer,
/// `aux` a second input used only on the tag-1 path. The tag's low 3 bits
/// select the path through a five-way dispatch: tag 1 normalises the three
/// floats at `aux` through a helper taking a frame-local buffer; tags 2 and 3
/// derive an angle from bits 15..23 of the tag word (`byte * 2π/255`, the
/// scale read from the game's read-only data), take sine and cosine through
/// helpers, pick an arctangent branch from the child's matrix at `+0x20` by
/// comparing each axis magnitude against 0.9, and combine the two
/// sine/cosine pairs into a rotated heading; tags 4 and 5 write the
/// negated sine, the cosine and zero directly. Anything else, a null child,
/// or (on tag 1) a null `aux` writes `(0, 1, 0)`. Returns `out`.
///
/// The arctangent helpers take their doubles in vector registers, which the
/// checker cannot forward to a Rust rewrite, so the contract routes each call
/// site to its own scripted answer (the branch taken stays observable) and
/// carries the answer to the rewrite through a scratch slot past the node;
/// the rewrite converts it exactly as the original's double-to-float
/// instruction does. The first cosine call's vector input is likewise left
/// uncompared (its upper lanes carry residue the transport zeroes); the same
/// angle is verified through the sine call made with it.
///
/// Original: 0x00C8ED10 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00C8ED10(this: u32, out: u32, aux: u32) -> u32 {
    unsafe {
        const OFF_CHILD: u32 = 0x10;
        const OFF_MAT: u32 = 0x20;
        const OFF_FALLBACK: u32 = 0x1c;
        const ANS_SCRATCH: u32 = 0x40;
        const ANGLE_SHIFT: u32 = 15;
        const ANGLE_SCALE: u32 = 0x00ED6D24;
        const THRESH_BITS: u32 = 0x3F666666; // 0.9, the axis-dominance cutoff
        const SIGN_BIT: u32 = 0x8000_0000;
        const CAL_SIN: u32 = 1;
        const CAL_COS1: u32 = 2;
        const CAL_COS2: u32 = 3;
        const CAL_ATAN_A: u32 = 4;
        const CAL_ATAN_B: u32 = 5;
        const CAL_NORM: u32 = 6;

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
        /// Absolute value exactly as the original's compare-and-xor sequence:
        /// negate only when `0.0 > v` (false for NaN and signed zeros).
        #[inline(always)]
        fn abs_select(v: f32) -> f32 {
            if 0.0f32 > v {
                negate_bits(v)
            } else {
                v
            }
        }
        /// The scripted arctangent answer, converted exactly as the
        /// original's double-to-float instruction converts it.
        #[inline(always)]
        unsafe fn atan_float(scratch_base: u32) -> f32 {
            unsafe {
                let lo = rd32(scratch_base + ANS_SCRATCH) as u64;
                let hi = rd32(scratch_base + ANS_SCRATCH + 4) as u64;
                core::hint::black_box(f64::from_bits((hi << 32) | lo)) as f32
            }
        }

        let idx = (rd32(this) & 7).wrapping_sub(1);
        if idx > 4 {
            wr32(out, 0);
            wr32(out + 0x04, 0x3F800000);
            wr32(out + 0x08, 0);
            return out;
        }
        // Jump-table mapping of idx to path: 0 normalise, 1-2 track, 3-4 direct.
        if idx == 0 {
            if rd32(this + OFF_CHILD) == 0 || aux == 0 {
                wr32(out, 0);
                wr32(out + 0x04, 0x3F800000);
                wr32(out + 0x08, 0);
                return out;
            }
            let mut b = [0u32; 3];
            b[0] = rd32(aux);
            b[1] = rd32(aux + 0x04);
            b[2] = rd32(aux + 0x08);
            lf_checker_rt::callee_thiscall!(CAL_NORM, u32, b.as_mut_ptr() as u32);
            wr32(out, b[0]);
            wr32(out + 0x04, b[1]);
            wr32(out + 0x08, b[2]);
            return out;
        }
        if idx >= 3 {
            let byte = ((rd32(this) >> ANGLE_SHIFT) & 0xFF) as u8;
            let angle = mul(byte as i32 as f32, rdf(lf_checker_rt::relocated(ANGLE_SCALE)));
            let sin = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_SIN, u32, angle.to_bits()));
            wrf(out, negate_bits(sin));
            let cos = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_COS1, u32,));
            wrf(out + 0x04, cos);
            wr32(out + 0x08, 0);
            return out;
        }
        let child = rd32(this + OFF_CHILD);
        if child == 0 {
            wr32(out, 0);
            wr32(out + 0x04, 0x3F800000);
            wr32(out + 0x08, 0);
            return out;
        }
        let byte = ((rd32(this) >> ANGLE_SHIFT) & 0xFF) as u8;
        let angle = mul(byte as i32 as f32, rdf(lf_checker_rt::relocated(ANGLE_SCALE)));
        let sin1 = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_SIN, u32, angle.to_bits()));
        let nsin1 = negate_bits(sin1);
        let cos1 = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_COS1, u32,));
        let mat = rd32(child + OFF_MAT);
        let thresh = f32::from_bits(THRESH_BITS);
        let a: f32;
        if abs_select(rdf(mat + 0x28)) > thresh {
            if mat == 0 {
                a = rdf(child + OFF_FALLBACK);
            } else {
                lf_checker_rt::callee_thiscall!(CAL_ATAN_B, u32, this);
                a = atan_float(this);
            }
        } else if abs_select(rdf(mat + 0x18)) > thresh {
            lf_checker_rt::callee_thiscall!(CAL_ATAN_A, u32, this);
            a = atan_float(this);
        } else if abs_select(rdf(mat + 0x08)) > thresh {
            lf_checker_rt::callee_thiscall!(CAL_ATAN_A, u32, this);
            a = atan_float(this);
        } else if mat == 0 {
            a = rdf(child + OFF_FALLBACK);
        } else {
            lf_checker_rt::callee_thiscall!(CAL_ATAN_B, u32, this);
            a = atan_float(this);
        }
        let sin2 = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_SIN, u32, a.to_bits()));
        let cos2 = f32::from_bits(lf_checker_rt::callee_cdecl!(CAL_COS2, u32, a.to_bits()));
        let out1 = add(mul(cos2, cos1), mul(sin2, nsin1));
        let out0 = sub(mul(cos2, nsin1), mul(sin2, cos1));
        wr32(out + 0x08, 0);
        wrf(out + 0x04, out1);
        wrf(out, out0);
        out
    }
});
