// original: 0x008F6790 Input_ResetDevice

/// Reset the device record at `this`: kind word at `+4` becomes 1, flag byte
/// at `+0` and state word at `+8` become 0. Returns 0. Convention: thiscall,
/// no stack words.
lf_checker_rt::export!(thiscall, rw_008f6790(this: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 4;
        const FLAG_OFF: u32 = 0;
        const STATE_OFF: u32 = 8;
        (this.wrapping_add(KIND_OFF) as *mut u32).write_unaligned(1);
        (this.wrapping_add(FLAG_OFF) as *mut u8).write(0);
        (this.wrapping_add(STATE_OFF) as *mut u16).write_unaligned(0);
        0
    }
});
