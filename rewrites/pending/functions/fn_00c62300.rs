// original: 0x00c62300 CAnimPlayer::vf1
/// First animation-player virtual: gated scaled rate.
///
/// Returns the field at `+0x2C` unless it equals the global sentinel, in which
/// case returns the field at `+0x60` multiplied once by the global rate.
export!(thiscall, rw_00c62300(this: u32) -> f32 {
    unsafe {
        let current = f32::from_bits(*((this + 0x2C) as *const u32));
        let gate = f32::from_bits(*global::<u32>(0xFE8D94));
        if gate == current {
            let base = f32::from_bits(*((this + 0x60) as *const u32));
            let rate = f32::from_bits(*global::<u32>(0x11735BC));
            // mulss with the original's operand order. NaN selects explicitly:
            // the backend commutes a plain multiply (verified in the built DLL),
            // which would pick the other NaN's sign/payload when both are NaN.
            if base.is_nan() {
                f32::from_bits(base.to_bits() | 0x0040_0000)
            } else if rate.is_nan() {
                f32::from_bits(rate.to_bits() | 0x0040_0000)
            } else {
                base * rate
            }
        } else {
            current
        }
    }
});
