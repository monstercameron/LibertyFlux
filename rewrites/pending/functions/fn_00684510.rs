// original: 0x00684510 rage::crFrameDofFloat::vf10
/// Float degree-of-freedom reset step: clears the validity bit and zeroes
/// the live value.
export!(thiscall, rw_00684510(this_: *mut u8) -> u32 {
    unsafe {
        *this_.add(TAG_OFF) &= !FLAG;
        *(this_.add(VALUE_OFF) as *mut u32) = 0;
        0
    }
});
