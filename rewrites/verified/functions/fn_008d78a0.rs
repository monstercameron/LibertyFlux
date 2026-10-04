// original: 0x008d78a0 clamp_unit_and_shape
// Clamps the input into [-1, 1] (out-of-range values snap to the nearer
// bound, NaN passes through) and returns the shaped value.
export!(cdecl, rw_008d78a0(x: f32) -> f32 {
    unsafe {
        let hi = *global::<f32>(0x00FE_88E8);
        let lo = *global::<f32>(0x00FE_8D94);
        let c = if x > hi {
            hi
        } else if lo > x {
            lo
        } else {
            x
        };
        f32::from_bits(callee_cdecl!(1, u32, c.to_bits()))
    }
});
