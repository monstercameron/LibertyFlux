// original: 0x00bf3600 blend_object_rows (proposed name)
/// Blend an object's stored row floats toward a source vector.
///
/// `this_` is the object (key at +0x1c, three row floats at
/// +0x20/+0x24/+0x28), `out` the destination (+0x30..+0x3c receive the
/// blended rows and a helper value), `vec` a 4-float orientation, `src` a
/// 3-float source vector, and `blend` the blend factor. A helper fills a
/// 4-float direction vector (negated when it opposes the orientation), two
/// more helpers consume it, and the rows become
/// `stored * (1 - factor) + src * factor`. Returns the source pointer.
export!(thiscall, rw_00bf3600(this_: *mut u8, out: *mut u8, vec: *const f32, src: *const f32, blend: f32) -> u32 {
    unsafe {
        // Scratch vector: the first helper fills words 0..6; words 0..4
        // are the direction vector (negated below when it opposes the
        // orientation) and words 4..8 are the second helper's object
        // (its fourth word is filled by it).
        let mut w = [0u32; 8];
        let key = *(this_.add(0x1c) as *const u32);
        callee_cdecl!(1, u32, w.as_mut_ptr() as u32, key);
        // The original reads these four words before its `(an instruction of the original)`, so
        // they are words 0..4 of the filled vector.
        let w0 = f32::from_bits(w[0]);
        let w1 = f32::from_bits(w[1]);
        let w2 = f32::from_bits(w[2]);
        let w3 = f32::from_bits(w[3]);
        let q0 = *vec;
        let q1 = *vec.add(1);
        let q2 = *vec.add(2);
        let q3 = *vec.add(3);
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
        callee_thiscall!(2, u32, t, blend.to_bits(), w.as_mut_ptr() as u32, vec as u32);
        callee_thiscall!(3, u32, out as u32, t);
        // Copy the stored rows, then blend toward the source vector.
        let r20 = *(this_.add(0x20) as *const u32);
        let r24 = *(this_.add(0x24) as *const u32);
        let r28 = *(this_.add(0x28) as *const u32);
        *(out.add(0x30) as *mut u32) = r20;
        *(out.add(0x34) as *mut u32) = r24;
        *(out.add(0x38) as *mut u32) = r28;
        let o30 = *(out.add(0x30) as *const f32);
        let inv = 1.0f32 - blend;
        let s0 = *src;
        let s1 = *src.add(1);
        let s2 = *src.add(2);
        let p0 = s0 * blend;
        let p2 = s2 * blend;
        let p1 = s1 * blend;
        let o34 = *(out.add(0x34) as *const f32);
        let o38 = *(out.add(0x38) as *const f32);
        let r0 = inv * o34;
        let r1 = o30 * inv;
        let r2 = inv * o38;
        let tail = f32::from_bits(w[7]);
        *(out.add(0x30) as *mut f32) = r1 + p0;
        *(out.add(0x34) as *mut f32) = r0 + p1;
        *(out.add(0x38) as *mut f32) = r2 + p2;
        *(out.add(0x3c) as *mut f32) = tail;
        src as u32
    }
});
