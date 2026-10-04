// original: 0x00c624f0 CAnimPlayer::vf2
/// Second animation-player virtual: shaped and rescaled rate.
///
/// Returns the field at `+0x30` unless it equals the global sentinel. Otherwise
/// shapes the bound animation's duration (scaled, then nudged up and back down
/// by the global unit), clamps the shaping result from below by that unit,
/// and returns the field at `+0x54` times the global rate times the shaping
/// scale over the clamped value.
export!(thiscall, rw_00c624f0(this: u32) -> f32 {
    unsafe {
        let current = f32::from_bits(*((this + 0x30) as *const u32));
        let gate = f32::from_bits(*global::<u32>(0xFE8D94));
        if gate != current {
            return current;
        }
        let anim = *((this + 0x40) as *const u32);
        let span = f32::from_bits(*((anim + 0xC) as *const u32));
        let scale = f32::from_bits(*global::<u32>(0xFE8B48));
        let floor = f32::from_bits(*global::<u32>(0xFE88E8));
        // Each multiply/add below selects NaN operands explicitly, in the
        // original's operand order: the backend commutes a plain mulss/addss
        // (verified in the built DLL), which would pick the other NaN's sign
        // and payload when both operands are NaN. Subtract and divide cannot
        // be commuted and stay plain.
        let mut shaped = if span.is_nan() {
            f32::from_bits(span.to_bits() | 0x0040_0000)
        } else if scale.is_nan() {
            f32::from_bits(scale.to_bits() | 0x0040_0000)
        } else {
            span * scale
        };
        shaped = if shaped.is_nan() {
            f32::from_bits(shaped.to_bits() | 0x0040_0000)
        } else if floor.is_nan() {
            f32::from_bits(floor.to_bits() | 0x0040_0000)
        } else {
            shaped + floor
        };
        shaped = shaped - floor;
        let denom = if floor > shaped { floor } else { shaped };
        let base = f32::from_bits(*((this + 0x54) as *const u32));
        let rate = f32::from_bits(*global::<u32>(0x11735BC));
        let prod = if base.is_nan() {
            f32::from_bits(base.to_bits() | 0x0040_0000)
        } else if rate.is_nan() {
            f32::from_bits(rate.to_bits() | 0x0040_0000)
        } else {
            base * rate
        };
        let quot = scale / denom;
        if prod.is_nan() {
            f32::from_bits(prod.to_bits() | 0x0040_0000)
        } else if quot.is_nan() {
            f32::from_bits(quot.to_bits() | 0x0040_0000)
        } else {
            prod * quot
        }
    }
});
