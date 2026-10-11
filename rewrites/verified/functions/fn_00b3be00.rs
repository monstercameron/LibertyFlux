// original: 0x00B3BE00 store_two_global_scalars
/// Store two single-precision global setting words in parameter order. Both
/// values are copied as bits so special values remain unchanged.
lf_checker_rt::export!(cdecl, rw_00b3be00(first_bits: u32, second_bits: u32) -> () {
    unsafe {
        const FIRST_SETTING: u32 = 0x0104592c;
        const SECOND_SETTING: u32 = 0x01045930;
        lf_checker_rt::global::<u32>(FIRST_SETTING).write_unaligned(first_bits);
        lf_checker_rt::global::<u32>(SECOND_SETTING).write_unaligned(second_bits);
    }
});
