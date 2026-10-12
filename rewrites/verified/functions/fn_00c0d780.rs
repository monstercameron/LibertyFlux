// original: 0x00C0D780 UILayoutFrame::are_two_flags_set

/// Returns one only when both the byte flag at receiver offset 0xD0 and the
/// byte flag at offset 0xC5 are nonzero. Thiscall receives the object in ECX.
lf_checker_rt::export!(thiscall, rw_c0d780(this: u32) -> u32 {
    const PRIMARY_FLAG: u32 = 0xD0;
    const SECONDARY_FLAG: u32 = 0xC5;
    let primary = unsafe { (this.wrapping_add(PRIMARY_FLAG) as *const u8).read() };
    if primary == 0 {
        return 0;
    }
    let secondary = unsafe { (this.wrapping_add(SECONDARY_FLAG) as *const u8).read() };
    u32::from(secondary != 0)
});
