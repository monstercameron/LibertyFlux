// original: 0x00684520 rage::crFrameDofFloat::vf11
/// Float degree-of-freedom invalidate step: sets the validity bit and fills
/// the live value with the 0x7F800001 bit pattern.
export!(thiscall, rw_00684520(this_: *mut u8) -> u32 {
    unsafe {
        *this_.add(TAG_OFF) |= FLAG;
        *(this_.add(VALUE_OFF) as *mut u32) = 0x7F80_0001;
        0
    }
});
