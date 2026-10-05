// original: 0x009D22D0 flag_bytes_setter (proposed)
//
/// Stores the low byte of each of four arguments into four slots.
///
/// Writes `a0..a3` (low byte only; upper bytes ignored) to `this+0x38` ..
/// `this+0x3b`. The low byte of the return value is the last byte stored;
/// the upper 24 bits are whatever the caller had in `eax` (the original
/// never touches them), so only `al` is compared. Thiscall, four stack words.
lf_checker_rt::export!(thiscall, rw_009D22D0(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x38;
        (this.wrapping_add(SLOTS) as *mut u8).write(a0 as u8);
        (this.wrapping_add(SLOTS + 1) as *mut u8).write(a1 as u8);
        (this.wrapping_add(SLOTS + 2) as *mut u8).write(a2 as u8);
        (this.wrapping_add(SLOTS + 3) as *mut u8).write(a3 as u8);
        a3 & 0xff
    }
});
