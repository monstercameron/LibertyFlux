// original: 0x00a4ec10 CVehicle::vf85 (symbols)

/// Whether the flag is set and the slot differs from the global threshold.
///
/// Returns 1 only when bit 3 of the flag byte at +0xf14 is set and the
/// float at +0x1078 is not equal to the global float (an unordered NaN
/// comparison also yields 1; only exact equality, including -0.0 == +0.0,
/// yields 0). The lahf/test/jnp shape implements not-equal. Thiscall, no
/// stack words, 1 or 0 in al (upper eax is entry garbage plus flags).
lf_checker_rt::export!(thiscall, rw_00a4ec10(this: u32) -> u32 {
    unsafe {
        const FLAG_OFF: u32 = 0xf14;
        const FLAG_BIT: u8 = 0x08;
        const VAL_OFF: u32 = 0x1078;
        const THR_VA: u32 = 0x00fe8628;
        let flag = ((this.wrapping_add(FLAG_OFF)) as *const u8).read();
        if flag & FLAG_BIT == 0 {
            return 0;
        }
        let v = f32::from_bits((this.wrapping_add(VAL_OFF) as *const u32).read_unaligned());
        let t = f32::from_bits((lf_checker_rt::relocated(THR_VA) as *const u32).read_unaligned());
        if v != t {
            1
        } else {
            0
        }
    }
});
