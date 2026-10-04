// original: 0x00684750 rage::crFrameDofInt::vf9
/// Integer degree-of-freedom exchange step: swaps the live values; this
/// object's tag takes the other's tag with the validity bit cleared while
/// the other takes this object's tag verbatim. Returns the old value with
/// its low byte replaced by the cleared other tag, matching EAX.
export!(thiscall, rw_00684750(this_: *mut u8, other: *mut u8) -> u32 {
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
