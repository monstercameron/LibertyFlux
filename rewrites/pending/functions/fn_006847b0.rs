// original: 0x006847b0 rage::crFrameDofInt::vf14
/// Integer degree-of-freedom assign step: clears the validity bit, stores
/// the given integer, and returns it.
export!(thiscall, rw_006847b0(this_: *mut u8, value: u32) -> u32 {
    unsafe {
        *this_.add(TAG_OFF) &= !FLAG;
        *(this_.add(VALUE_OFF) as *mut u32) = value;
        value
    }
});
