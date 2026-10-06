// original: 0x00dd9280 UIBasicClip::vf143

/// Pointer to the clip's field at +0x1f8.
///
/// `this` is the clip object. The function returns `this + FIELD_1F8`
/// unchanged otherwise: no reads, no writes, no calls.
///
/// Original: thiscall, no stack arguments, address returned in EAX.
lf_checker_rt::export!(thiscall, rw_00dd9280(this: u32) -> u32 {
    const FIELD_1F8: u32 = 0x1f8;
    this.wrapping_add(FIELD_1F8)
});
