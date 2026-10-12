// original: 0x00C0D7B0 UILayoutFrame::vf42

/// Returns one when the 32-bit field at receiver offset 0xB4 is nonzero,
/// otherwise returns zero. The method takes its receiver in ECX.
lf_checker_rt::export!(thiscall, rw_c0d7b0(this: u32) -> u32 {
    const STATE_FIELD: u32 = 0xB4;
    let state = unsafe { (this.wrapping_add(STATE_FIELD) as *const u32).read_unaligned() };
    u32::from(state != 0)
});
