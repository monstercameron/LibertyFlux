// original: 0x00b5bbf0 zero16_set_flag (proposed)

/// Initialise the first 17 bytes of the object at `this`: words at `+0x00`
/// through `+0x0c` to zero, byte at `+0x19` to 1, and the word at `+0x14`
/// to all ones. Returns `this`.
lf_checker_rt::export!(thiscall, rw_00b5bbf0(this: u32) -> u32 {
    unsafe {
        const ZERO_END: u32 = 0x10;
        const FLAG: u32 = 0x19;
        const MARK: u32 = 0x14;
        for off in (0..ZERO_END).step_by(4) {
            (this.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(FLAG) as *mut u8).write(1);
        (this.wrapping_add(MARK) as *mut u32).write_unaligned(0xffff_ffff);
        this
    }
});
