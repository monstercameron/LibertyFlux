// original: 0x00604AB0 rage::spdApical::vf5
/// Store the incoming single-precision value's raw bits in the `field_1c`
/// member at offset `+0x1c`. The setter performs no arithmetic and has no
/// meaningful integer return value.
lf_checker_rt::export!(thiscall, spd_apical_vf5(object: u32, value_bits: u32) -> () {
    const FIELD_1C: u32 = 0x1c;
    unsafe {
        core::ptr::write_volatile(
            object.wrapping_add(FIELD_1C) as *mut u32,
            value_bits,
        );
    }
});
