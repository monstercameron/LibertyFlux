// original: 0x00684480 rage::crFrameDofFloat::vf6
/// Float degree-of-freedom "adopt if larger" step.
///
/// If the square of the candidate value exceeds the square of the given
/// limit, copies the candidate value into this object and copies the
/// candidate's validity bit into this object's tag byte. Returns the
/// candidate pointer, matching the value the original leaves in EAX.
export!(thiscall, rw_00684480(this_: *mut u8, other: *const u8, limit: f32) -> u32 {
    unsafe {
        let cand = *(other.add(VALUE_OFF) as *const f32);
        if cand * cand > limit * limit {
            *(this_.add(VALUE_OFF) as *mut f32) = cand;
            if *other.add(TAG_OFF) & FLAG != 0 {
                *this_.add(TAG_OFF) |= FLAG;
            } else {
                *this_.add(TAG_OFF) &= !FLAG;
            }
        }
        other as u32
    }
});
