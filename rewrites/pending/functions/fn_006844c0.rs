// original: 0x006844c0 rage::crFrameDofFloat::vf7
/// Float degree-of-freedom scale step: multiplies the live value by the
/// given factor.
export!(thiscall, rw_006844c0(this_: *mut u8, factor: f32) -> u32 {
    unsafe {
        let slot = this_.add(VALUE_OFF) as *mut f32;
        *slot = *slot * factor;
        0
    }
});
