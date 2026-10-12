// original: 0x00b5c170 init_sentinels (proposed)

/// Initialise an object at `this`: the head word to 0, the word at `+0x20`
/// and the words at `+0x28`/`+0x2c` to all ones (-1), and the byte at
/// `+0x24` to 0. Returns `this`.
lf_checker_rt::export!(thiscall, rw_00b5c170(this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x00;
        const FIRST: u32 = 0x20;
        const FLAG: u32 = 0x24;
        const SECOND: u32 = 0x28;
        const THIRD: u32 = 0x2c;
        (this.wrapping_add(HEAD) as *mut u32).write_unaligned(0);
        (this.wrapping_add(FIRST) as *mut u32).write_unaligned(0xffff_ffff);
        (this.wrapping_add(FLAG) as *mut u8).write(0);
        (this.wrapping_add(SECOND) as *mut u32).write_unaligned(0xffff_ffff);
        (this.wrapping_add(THIRD) as *mut u32).write_unaligned(0xffff_ffff);
        this
    }
});
