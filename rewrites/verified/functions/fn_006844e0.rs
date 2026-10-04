// original: 0x006844e0 rage::crFrameDofFloat::vf9
/// Float degree-of-freedom exchange step.
///
/// Swaps the live values of the two objects. This object's tag byte takes
/// the other object's tag with the validity bit cleared; the other object's
/// tag takes this object's tag verbatim. Returns the old live value of this
/// object with its low byte replaced by the cleared other tag, matching EAX.
export!(thiscall, rw_006844e0(this_: *mut u8, other: *mut u8) -> u32 {
    unsafe {
        let this_val = *(this_.add(VALUE_OFF) as *const u32);
        let other_val = *(other.add(VALUE_OFF) as *const u32);
        let this_tag = *this_.add(TAG_OFF);
        let other_tag = *other.add(TAG_OFF) & !FLAG;
        *(other.add(VALUE_OFF) as *mut u32) = this_val;
        *(this_.add(VALUE_OFF) as *mut u32) = other_val;
        *this_.add(TAG_OFF) = other_tag;
        *other.add(TAG_OFF) = this_tag;
        (this_val & 0xFFFF_FF00) | other_tag as u32
    }
});
