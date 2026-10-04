// original: 0x00684550 rage::crFrameDofFloat::vf14
/// Float degree-of-freedom assign step: clears the validity bit and stores
/// the given value.
export!(thiscall, rw_00684550(this_: *mut u8, value: f32) -> u32 {
    unsafe {
        *this_.add(TAG_OFF) &= !FLAG;
        *(this_.add(VALUE_OFF) as *mut f32) = value;
        0
    }
});
