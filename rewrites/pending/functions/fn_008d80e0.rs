// original: 0x008d80e0 lerp_vec3
/// Component-wise lerp of two 3-vectors: `out[i] = a[i] + (b[i] - a[i]) * t`.
export!(cdecl, rw_008d80e0(a: *const f32, b: *const f32, t: f32, out: *mut f32) -> u32 {
    unsafe {
    /// Scalar op with the original's exact NaN routing: a quiet-NaN dest
    /// wins, else a quiet-NaN src, else the real operation. A plain Rust op
    /// lets LLVM swap commutative operands, which picks the other payload
    /// when both are NaN. Op codes: 0 = add, 1 = sub, 2 = mul, 3 = div.
    let sse = |dest: f32, src: f32, op: u8| -> f32 {
        if dest.is_nan() {
            dest
        } else if src.is_nan() {
            src
        } else {
            match op {
                0 => dest + src,
                1 => dest - src,
                2 => dest * src,
                _ => dest / src,
            }
        }
    };

        for i in 0..3 {
            let av = *a.add(i);
            let bv = *b.add(i);
            let d = sse(bv, av, 1);
            let m = sse(d, t, 2);
            *out.add(i) = sse(m, av, 0);
        }
        // The original leaves the output pointer in EAX.
        out as u32
    }
});
