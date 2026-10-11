// original: 0x00B3BDF0 NativeImpl_SET_PED_DENSITY_MULTIPLIER
/// Store the supplied single-precision multiplier bit pattern in the global
/// native setting word. The transfer is a bitwise copy, with no rounding.
lf_checker_rt::export!(cdecl, rw_00b3bdf0(multiplier_bits: u32) -> () {
    unsafe {
        const MULTIPLIER_SETTING: u32 = 0x01045928;
        lf_checker_rt::global::<u32>(MULTIPLIER_SETTING).write_unaligned(multiplier_bits);
    }
});
