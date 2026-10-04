// original: 0x006846f0 rage::crFrameDofInt::vf6
/// Integer degree-of-freedom "adopt if larger" step.
///
/// Compares the square of the candidate (converted to float) against the
/// square of the given limit; on strictly greater, copies the candidate
/// integer and its validity bit into this object. Returns the candidate
/// pointer, matching EAX.
export!(thiscall, rw_006846f0(this_: *mut u8, other: *const u8, limit: f32) -> u32 {
    unsafe {
        let cand = *(other.add(VALUE_OFF) as *const i32);
        let f = cand as f32;
        if f * f > limit * limit {
            *(this_.add(VALUE_OFF) as *mut i32) = cand;
            if *other.add(TAG_OFF) & FLAG != 0 {
                *this_.add(TAG_OFF) |= FLAG;
            } else {
                *this_.add(TAG_OFF) &= !FLAG;
            }
        }
        other as u32
    }
});
