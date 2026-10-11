// original: 0x00DE49C0 scrolling_row_is_available
/// Return zero when the row's disabled byte at +0x214 is nonzero. Otherwise
/// return one exactly when the unsigned count at +0x1FC is nonzero. The method
/// is thiscall and reads no other row state.
lf_checker_rt::export!(thiscall, rw_de49c0(this: u32) -> u8 {
    const DISABLED: u32 = 0x214;
    const COUNT: u32 = 0x1FC;
    unsafe {
        let disabled = ((this + DISABLED) as *const u8).read();
        if disabled != 0 {
            0
        } else {
            u8::from(((this + COUNT) as *const u32).read_unaligned() != 0)
        }
    }
});

