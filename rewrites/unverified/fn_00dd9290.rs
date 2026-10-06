// original: 0x00dd9290 UIBasicClip::vf148

/// Float field of the clip at +0x2fc.
///
/// `this` is the clip object. One float is loaded from `FIELD_2FC (+0x2fc)`
/// and returned on the x87 stack; nothing is written and no calls are made.
/// The bits pass through untouched, so NaNs keep sign and payload.
///
/// Original: thiscall, no stack arguments, float result left in ST0.
lf_checker_rt::export!(thiscall, rw_00dd9290(this: u32) -> f32 {
    const FIELD_2FC: u32 = 0x2fc;
    unsafe { f32::from_bits(((this + FIELD_2FC) as *const u32).read_unaligned()) }
});
