// original: 0x00604AA0 rage::spdApical::vf4
/// Store the incoming single-precision value's raw bits in the `field_18`
/// member at offset `+0x18`. The setter performs no arithmetic and has no
/// meaningful integer return value.
lf_checker_rt::export!(thiscall, spd_apical_vf4(object: u32, value_bits: u32) -> () {
    const FIELD_18: u32 = 0x18;
    unsafe {
        core::ptr::write_volatile(
            object.wrapping_add(FIELD_18) as *mut u32,
            value_bits,
        );
    }
});
