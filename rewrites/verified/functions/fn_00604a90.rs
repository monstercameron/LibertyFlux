// original: 0x00604A90 rage::spdApical::vf3
/// Store the incoming single-precision value's raw bits in the `field_14`
/// member at offset `+0x14`. The setter performs no arithmetic and has no
/// meaningful integer return value.
lf_checker_rt::export!(thiscall, spd_apical_vf3(object: u32, value_bits: u32) -> () {
    const FIELD_14: u32 = 0x14;
    unsafe {
        core::ptr::write_volatile(
            object.wrapping_add(FIELD_14) as *mut u32,
            value_bits,
        );
    }
});
