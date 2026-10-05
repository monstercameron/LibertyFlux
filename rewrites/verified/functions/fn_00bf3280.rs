// original: 0x00bf3280 blend_object_rotation (proposed name)
/// Blend an object's stored rotation rows toward a source vector.
///
/// `this_` is the object (key at +4, three row floats at +8/+0xc/+0x10),
/// `out` the destination (+0x30..+0x3c receive the blended rows and a helper
/// value), `quat` a 4-float orientation, `src` a 3-float source vector, and
/// `blend` the blend factor. When the source vector is all zero and the
/// orientation is within a small box of the identity quaternion, the factor
/// is forced to zero; otherwise the passed factor is used. A helper fills a
/// 4-float direction vector (negated when it opposes the orientation), two
/// more helpers consume it, and the rows become
/// `stored * (1 - factor) + src * factor`.
export!(thiscall, rw_00bf3280(this_: *mut u8, out: *mut u8, quat: *const f32, src: *const f32, blend: f32) -> u32 {
    unsafe {
        // Identity-quaternion box centre (read-only game constants) and radius.
        let centre = [
            f32::from_bits(0x00000000),
            f32::from_bits(0x00000000),
            f32::from_bits(0x00000000),
            f32::from_bits(0x3F800000),
        ];
        let radius = f32::from_bits(0x3A83126F);
        // Nonzero (or NaN) source component, or an out-of-box orientation
        // component, selects the passed blend factor; otherwise zero.
        let mut use_blend = *src != 0.0 || *src.add(1) != 0.0 || *src.add(2) != 0.0;
        if !use_blend {
            let mut i = 0;
            while i < 4 {
                let q = *quat.add(i);
                let lo = centre[i] - radius;
                if !(q >= lo) {
                    use_blend = true;
                    break;
                }
                let hi = centre[i] + radius;
                if !(q <= hi) {
                    use_blend = true;
                    break;
                }
                i += 1;
            }
        }
        let factor = if use_blend { blend } else { 0.0 };
        // Scratch vector: the first helper fills words 0..6; words 0..4
        // are the direction vector (negated below when it opposes the
        // orientation) and words 4..8 are the second helper's object
        // (its fourth word is filled by it).
        let mut w = [0u32; 8];
        let key = *(this_.add(4) as *const u32);
        callee_cdecl!(1, u32, w.as_mut_ptr() as u32, key);
        // The original reads these four words before its `(an instruction of the original)`, so
        // they are words 0..4 of the filled vector, not 2..6.
        let w0 = f32::from_bits(w[0]);
        let w1 = f32::from_bits(w[1]);
        let w2 = f32::from_bits(w[2]);
        let w3 = f32::from_bits(w[3]);
        let q0 = *quat;
        let q1 = *quat.add(1);
        let q2 = *quat.add(2);
        let q3 = *quat.add(3);
        let mut d = q1 * w1 + q0 * w0;
        d += q2 * w2;
        d += q3 * w3;
        if d < 0.0 {
            w[0] = (-w0).to_bits();
            w[1] = (-w1).to_bits();
            w[2] = (-w2).to_bits();
            w[3] = (-w3).to_bits();
        }
        let t = w.as_mut_ptr().add(4) as u32;
        callee_thiscall!(2, u32, t, factor.to_bits(), w.as_mut_ptr() as u32, quat as u32);
        callee_thiscall!(3, u32, out as u32, t);
        // Copy the stored rows, then blend toward the source vector.
        let r8 = *(this_.add(8) as *const u32);
        let r12 = *(this_.add(0x0c) as *const u32);
        let r16 = *(this_.add(0x10) as *const u32);
        *(out.add(0x30) as *mut u32) = r8;
        *(out.add(0x34) as *mut u32) = r12;
        *(out.add(0x38) as *mut u32) = r16;
        let s0 = *src;
        let s1 = *src.add(1);
        let s2 = *src.add(2);
        let o34 = *(out.add(0x34) as *const f32);
        let inv = 1.0f32 - factor;
        let p0 = s0 * factor;
        let p1 = s1 * factor;
        let p2 = s2 * factor;
        let o38 = *(out.add(0x38) as *const f32);
        let o30 = *(out.add(0x30) as *const f32);
        let r0 = inv * o30;
        let r1 = o34 * inv;
        let r2 = o38 * inv;
        let tail = f32::from_bits(w[7]);
        *(out.add(0x30) as *mut f32) = r0 + p0;
        *(out.add(0x34) as *mut f32) = r1 + p1;
        *(out.add(0x38) as *mut f32) = r2 + p2;
        *(out.add(0x3c) as *mut f32) = tail;
        r16
    }
});
