// original: 0x0059CC90 any_global_flag_set

/// Test whether either of two global flags is nonzero.
///
/// The first unsigned global is checked before the second, so the second
/// address is read only when the first value is zero. The cdecl function has
/// no arguments and returns its boolean in `AL`.
lf_checker_rt::export!(cdecl, rw_0059cc90() -> u32 {
    const PRIMARY_FLAG: u32 = 0x01168BC4;
    const SECONDARY_FLAG: u32 = 0x01168BC8;

    unsafe {
        if lf_checker_rt::global::<u32>(PRIMARY_FLAG).read_unaligned() != 0 {
            return 1;
        }
        u32::from(lf_checker_rt::global::<u32>(SECONDARY_FLAG).read_unaligned() != 0)
    }
});
