// original: 0x00c62560 anim_tagged_scale_product
/// Tagged scale product: the bound duration times the local multiplier.
///
/// When the tag at `+0x44` is 1, returns the duration at `+0xC` of the bound
/// animation times the multiplier at `+0x4C`. Any other tag makes the original
/// read through a null pointer, which faults; the rewrite performs the same
/// faulting read so both sides fault identically.
export!(thiscall, rw_00c62560(this: u32) -> f32 {
    unsafe {
        let tag = *((this + 0x44) as *const u16);
        // mulss with the original's operand order; NaN selects explicitly (see vf2).
        let (span, mult) = if tag == 1 {
            let anim = *((this + 0x40) as *const u32);
            (
                f32::from_bits(*((anim + 0xC) as *const u32)),
                f32::from_bits(*((this + 0x4C) as *const u32)),
            )
        } else {
            (
                core::ptr::read_volatile(0xC as *const f32),
                f32::from_bits(*((this + 0x4C) as *const u32)),
            )
        };
        if span.is_nan() {
            f32::from_bits(span.to_bits() | 0x0040_0000)
        } else if mult.is_nan() {
            f32::from_bits(mult.to_bits() | 0x0040_0000)
        } else {
            span * mult
        }
    }
});
