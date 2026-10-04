// original: 0x006f5510 init_record_arg1
/// Initialise a record with one argument and clear the remaining words.
///
/// Stores `a1` at `+0`, zeroes the words up to `+0x2C`, clears bit 0 at
/// `+0x32` and the word at `+0x30`. Returns `this`.
rt::export!(thiscall, rw_006f5510(this: *mut u8, a1: u32) -> u32 {
    unsafe {
        *this.cast::<u32>() = a1;
        for off in [4usize, 8, 0xC, 0x10, 0x14] {
            *this.add(off).cast::<u32>() = 0;
        }
        *this.add(0x32) &= 0xFE;
        *this.add(0x30).cast::<u16>() = 0;
        for off in [0x20usize, 0x24, 0x28, 0x2C] {
            *this.add(off).cast::<u32>() = 0;
        }
        this as u32
    }
});
