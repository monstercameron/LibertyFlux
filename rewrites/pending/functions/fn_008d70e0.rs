// original: 0x008d70e0 clamp_lerp
/// Clamp `p` into `[q, r]` and interpolate `s..=t` across the range.
///
/// Returns `s` when `q > p`, `t` when `p > r`, else the linear blend
/// `s + ((p - q) / (r - q)) * (t - s)`. NaN inputs fall through the ordered
/// comparisons exactly as `comiss`+`jbe` do.
#[allow(non_snake_case)]
export!(cdecl, rw_008d70e0(p: f32, q: f32, r: f32, s: f32, t: f32) -> f32 {
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
    // f32 return: rustc moves it to ST0 with fld, exactly like the original.
    if q > p {
        s
    } else if p > r {
        t
    } else {
        let d1 = sse(p, q, 1);
        let d2 = sse(r, q, 1);
        let u = sse(d1, d2, 3);
        let e1 = sse(t, s, 1);
        let m = sse(u, e1, 2);
        sse(m, s, 0)
    }
});
