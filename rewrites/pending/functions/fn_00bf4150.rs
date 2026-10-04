// original: 0x00bf4150 set_flags_hi
/// Fold two flag bits into the control byte at +0x10, then pack `v` high.
export!(thiscall, rw_bf4150(this: *mut u8, a0: u32, a1: u32, v: u32) -> u32 {
    unsafe {
        let mut dl = *this.add(0x10) & 0x47;
        dl |= ((a0 & 1) as u8) << 7;
        dl ^= ((a1 & 7) as u8) << 3;
        *this.add(0x10) = dl;
        callee_thiscall!(1, u32, this as u32, v)
    }
});
