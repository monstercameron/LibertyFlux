// original: 0x00b29270 frame_record_decode
/// Decode a packed frame record into an orthonormal basis plus payload.
///
/// The input record holds six signed bytes (offsets 0x0A..0x0F) that are
/// scaled by a global factor into two 3-vectors, plus three trailing dwords
/// (offsets 0x14..0x1C) copied verbatim to the output. The output receives
/// the two scaled vectors, their normalized cross product, a second vector
/// derived from them that is normalized as well, the cross products of the
/// normalized rows, and the three copied dwords. Normalization divides by
/// the vector length with a zero guard: a zero-length vector normalizes to
/// all zeros instead of dividing by zero. Returns the last copied dword.
export!(cdecl, rw_00b29270(out: u32, inp: u32) -> u32 {
    unsafe {
        const BYTE_SCALE: u32 = 0x00FE8700;
        const ONE: u32 = 0x00FE88E8;
        const OUT_LEN: u32 = 0x3C;
        let _ = OUT_LEN;
        let k: f32 = *global::<f32>(BYTE_SCALE);
        let one: f32 = *global::<f32>(ONE);
        let load_f = |base: u32, off: u32| -> f32 {
            *(base.wrapping_add(off) as *const f32)
        };
        let store_f = |base: u32, off: u32, v: f32| {
            *(base.wrapping_add(off) as *mut f32) = v;
        };
        // Six signed bytes scaled to floats; the two triples share the factor.
        let v0 = (*(inp.wrapping_add(0x0A) as *const i8) as f32) * k;
        let v1 = (*(inp.wrapping_add(0x0B) as *const i8) as f32) * k;
        let v2 = (*(inp.wrapping_add(0x0C) as *const i8) as f32) * k;
        let v3 = (*(inp.wrapping_add(0x0D) as *const i8) as f32) * k;
        let v4 = (*(inp.wrapping_add(0x0E) as *const i8) as f32) * k;
        let v5 = (*(inp.wrapping_add(0x0F) as *const i8) as f32) * k;
        store_f(out, 0x00, v0);
        store_f(out, 0x04, v1);
        store_f(out, 0x08, v2);
        store_f(out, 0x10, v3);
        store_f(out, 0x14, v4);
        store_f(out, 0x18, v5);
        // First derived row and its squared length.
        let t0 = v5 * v1 - v4 * v2;
        let t1 = v3 * v2 - v0 * v5;
        let t2 = v0 * v4 - v3 * v1;
        store_f(out, 0x24, t1);
        store_f(out, 0x28, t2);
        store_f(out, 0x20, t0);
        let len2 = t1 * t1 + t0 * t0 + t2 * t2;
        // Zero-guarded reciprocal length: zero stays zero, anything else
        // (including NaN) takes the divide path, as the original's flag test.
        let s = if len2 == 0.0 { 0.0 } else { one / len2.sqrt() };
        let st0 = s * t0;
        let st1 = s * t1;
        let st2 = t2 * s;
        store_f(out, 0x20, st0);
        store_f(out, 0x24, st1);
        store_f(out, 0x28, st2);
        // Second derived row from the normalized first row.
        let o0w = st2 * v4 - st1 * v5;
        store_f(out, 0x00, o0w);
        let o4w = v5 * st0 - st2 * v3;
        store_f(out, 0x04, o4w);
        let o8w = v3 * st1 - v4 * st0;
        store_f(out, 0x08, o8w);
        let len2b = o0w * o0w + o4w * o4w + o8w * o8w;
        let s2 = if len2b == 0.0 { 0.0 } else { one / len2b.sqrt() };
        let n8 = o8w * s2;
        let n4 = o4w * s2;
        let n0 = o0w * s2;
        store_f(out, 0x08, n8);
        store_f(out, 0x04, n4);
        store_f(out, 0x00, n0);
        // Remaining rows combine the normalized rows.
        store_f(out, 0x10, n8 * st1 - n4 * st2);
        store_f(out, 0x14, st2 * n0 - n8 * st0);
        store_f(out, 0x18, n4 * st0 - n0 * st1);
        // Trailing payload dwords copied verbatim.
        let w0 = *(inp.wrapping_add(0x14) as *const u32);
        let w1 = *(inp.wrapping_add(0x18) as *const u32);
        let w2 = *(inp.wrapping_add(0x1C) as *const u32);
        *(out.wrapping_add(0x30) as *mut u32) = w0;
        *(out.wrapping_add(0x34) as *mut u32) = w1;
        *(out.wrapping_add(0x38) as *mut u32) = w2;
        w2
    }
});
