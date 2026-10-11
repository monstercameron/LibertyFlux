// original: 0x00604A80 rage::spdApical::vf2
/// Store the incoming single-precision value's raw bits in the `field_10`
/// member at offset `+0x10`. The setter performs no arithmetic and has no
/// meaningful integer return value.
lf_checker_rt::export!(thiscall, spd_apical_vf2(object: u32, value_bits: u32) -> () {
    const FIELD_10: u32 = 0x10;
    unsafe {
        core::ptr::write_volatile(
            object.wrapping_add(FIELD_10) as *mut u32,
            value_bits,
        );
    }
});
