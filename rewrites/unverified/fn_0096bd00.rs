// original: 0x0096BD00 read_timing_state_outputs

/// Copies the object’s 32-bit state at `0x3004` and byte state at `0x3024`
/// through independently optional output pointers, then returns the byte at
/// `0x3000` in AL.
lf_checker_rt::export!(thiscall, rw_0096bd00(this: u32, word_out: u32, byte_out: u32) -> u32 {
    unsafe {
        if word_out != 0 {
            let value = (this.wrapping_add(0x3004) as *const u32).read_unaligned();
            (word_out as *mut u32).write_unaligned(value);
        }
        if byte_out != 0 {
            let value = (this.wrapping_add(0x3024) as *const u8).read();
            (byte_out as *mut u8).write(value);
        }
        u32::from((this.wrapping_add(0x3000) as *const u8).read())
    }
});
