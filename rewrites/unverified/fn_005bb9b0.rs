// original: 0x005BB9B0 both_frontend_flags_enabled
/// Return one only when bit 0 is set in both global state bytes; otherwise
/// return zero. The function does not modify either byte.
///
/// The original is a no-argument cdecl function with an 8-bit result.
lf_checker_rt::export!(cdecl, rw_005bb9b0() -> u8 {
    const FLAG_A_VA: u32 = 0x019d_3022;
    const FLAG_B_VA: u32 = 0x019d_302e;
    unsafe {
        let first = lf_checker_rt::global::<u8>(FLAG_A_VA).read();
        let second = lf_checker_rt::global::<u8>(FLAG_B_VA).read();
        u8::from(first & 1 != 0 && second & 1 != 0)
    }
});
