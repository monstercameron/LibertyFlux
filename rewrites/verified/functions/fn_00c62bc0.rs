// original: 0x00c62bc0 anim_player_update_blend
/// Blend updater: refreshes or clears the blend state for a signed weight.
///
/// For a positive weight within the blend helper's answer, or a negative
/// weight within the negated answer, forwards the weight to the blend writer
/// with full or zero strength. A non-positive weight that is not strictly
/// negative clears the blend flag instead. Returns the writer's answer, the
/// helper's answer bits, or 0xFFDF on the clearing path.
export!(thiscall, rw_00c62bc0(this: u32, arg: u32) -> u32 {
    unsafe {
        let a = f32::from_bits(arg);
        if a > 0.0 {
            let cap = f32::from_bits(*global::<u32>(0xFE88E8));
            let limit = f32::from_bits(*((this + 0x58) as *const u32));
            if cap > limit {
                return callee_thiscall!(1, u32, this, 0x3F80_0000, arg);
            }
            let got = callee_thiscall!(0, f32, this);
            if a > got {
                return callee_thiscall!(1, u32, this, 0x3F80_0000, arg);
            }
            return got.to_bits();
        }
        if 0.0 > a {
            let limit = f32::from_bits(*((this + 0x58) as *const u32));
            if limit > 0.0 {
                return callee_thiscall!(1, u32, this, 0, (-a).to_bits());
            }
            let got = callee_thiscall!(0, f32, this);
            if got > a {
                return callee_thiscall!(1, u32, this, 0, (-a).to_bits());
            }
            return got.to_bits();
        }
        *((this + 0x46) as *mut u16) &= 0xFFDF;
        0xFFDF
    }
});
