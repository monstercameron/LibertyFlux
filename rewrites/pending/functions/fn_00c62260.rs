// original: 0x00c62260 anim_blend_signed_weight
/// Signed blend weight for the animation player, gated by a flag bit and two sentinels.
///
/// Returns `+0.0` unless bit 5 of the flag byte at `+0x46` is set. When set and
/// the field at `+0x5C` equals the first global sentinel, returns the field at
/// `+0x60` (the original reaches shared code with a jump, making no call); when
/// it instead equals the second sentinel, returns the negation of the helper's
/// result; otherwise returns `+0.0`. Equality is ordered float equality, so NaN
/// never matches a sentinel.
export!(thiscall, rw_00c62260(this: u32) -> f32 {
    unsafe {
        let flags = *((this + 0x46) as *const u8);
        if flags & 0x20 == 0 {
            return 0.0;
        }
        let value = f32::from_bits(*((this + 0x5C) as *const u32));
        if value == f32::from_bits(*global::<u32>(0xFE88E8)) {
            return f32::from_bits(*((this + 0x60) as *const u32));
        }
        if value == f32::from_bits(*global::<u32>(0xFE8628)) {
            let helper = callee_thiscall!(0, f32, this);
            return -helper;
        }
        0.0
    }
});
