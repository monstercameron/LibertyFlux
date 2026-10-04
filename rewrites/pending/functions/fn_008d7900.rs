// original: 0x008d7900 scale_translate_pair
/// Scalar-SSE subtraction with the original's exact NaN rule: when either
/// input is a NaN, the destination (first) NaN wins, quieted. The backend is
/// free to swap the operands of a plain `-` on NaN inputs, which changes the
/// payload, so the role is emulated here. Non-NaN results are order-free.
fn sse_sub(dest: f32, src: f32) -> f32 {
    if dest.is_nan() {
        f32::from_bits(dest.to_bits() | 0x0040_0000)
    } else if src.is_nan() {
        f32::from_bits(src.to_bits() | 0x0040_0000)
    } else {
        dest - src
    }
}

// Applies an affine update to a two-float record: out[0] = t0 + m0*out[0]
// and out[1] = t1 + m1*out[1], where the (t, m) coefficients come from a
// shared lookup answered per call. Returns the last lookup answer.
export!(cdecl, rw_008d7900(pair: *mut f32) -> u32 {
    unsafe {
        let ctx = *global::<u32>(0x017F_583C);
        let a0: u32 = callee_thiscall!(1, u32, ctx);
        let t0 = *((a0 as *const f32).wrapping_add(0));
        let a1: u32 = callee_thiscall!(1, u32, ctx);
        // Destination-operand roles decide the NaN payload (see sse_sub), so
        // the multiply-add runs through the role-exact helpers.
        let acc0 = sse_add(
            sse_mul(*((a1 as *const f32).wrapping_add(2)), *pair),
            t0,
        );
        *pair = acc0;
        let a2: u32 = callee_thiscall!(1, u32, ctx);
        let t1 = *((a2 as *const f32).wrapping_add(1));
        let a3: u32 = callee_thiscall!(1, u32, ctx);
        let acc1 = sse_add(
            sse_mul(
                *((a3 as *const f32).wrapping_add(3)),
                *pair.wrapping_add(1),
            ),
            t1,
        );
        *pair.wrapping_add(1) = acc1;
        a3
    }
});
