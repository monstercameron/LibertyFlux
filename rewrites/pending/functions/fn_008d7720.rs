// original: 0x008d7720 wrap_blend_pair
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

// Blends two remapped inputs with wrap-around: each input is remapped and
// shifted by a constant, the first value is wrapped by one turn when the
// pair differs by more than half a turn, and the remap of the linear blend
// is left in the floating-point result register. Returns 0 as a placeholder
// (the real result travels in ST0, which safe Rust cannot name).
export!(cdecl, rw_008d7720(a: f32, b: f32, t: f32) -> u32 {
    unsafe {
        let shift = *global::<f32>(0x00E8_1214);
        let half_turn = *global::<f32>(0x00FE_8AA0);
        let full_turn = *global::<f32>(0x00FE_8AEC);
        let v0 = f32::from_bits(callee_cdecl!(1, u32, a.to_bits())) + shift;
        let v1 = f32::from_bits(callee_cdecl!(1, u32, b.to_bits())) + shift;
        let d = v0 - v1;
        let v0 = if d > half_turn {
            v0 - full_turn
        } else if d < -half_turn {
            v0 + full_turn
        } else {
            v0
        };
        // The original's destination-operand roles decide the NaN payload when
        // both inputs are NaN; the backend will not keep those roles for
        // plain operators (verified in the built DLL), so they are emulated.
        let diff = sse_sub(v1, v0);
        let prod = sse_mul(diff, t);
        let m = sse_add(prod, v0);
        let _r: u32 = callee_cdecl!(1, u32, m.to_bits());
        0
    }
});
