// original: 0x00684780 rage::crFrameDofInt::vf11
/// Integer degree-of-freedom invalidate step: sets the validity bit and
/// fills the live value with 0x80000000.
export!(thiscall, rw_00684780(this_: *mut u8) -> u32 {
    unsafe {
        *this_.add(TAG_OFF) |= FLAG;
        *(this_.add(VALUE_OFF) as *mut u32) = 0x8000_0000;
        0
    }
});
