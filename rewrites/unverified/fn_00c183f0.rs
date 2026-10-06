// original: 0x00c183f0 store_three_floats_set_flag

/// Store three argument floats into the object and mark it initialised.
///
/// Copies the three stack floats to `+0x2c0`, `+0x2c4`, `+0x2c8` of `this`,
/// then sets bit 0 and clears bit 1 of the flag byte at `+0x39b`. Returns the
/// new flag byte in `al` (the original leaves it there).
///
/// Original: 0x00C183F0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00c183f0(this: u32, f0: u32, f1: u32, f2: u32) -> u32 {
    unsafe {
        const VEC_OFF: u32 = 0x2c0;
        const FLAG_OFF: u32 = 0x39b;
        ((this + VEC_OFF) as *mut u32).write_unaligned(f0);
        ((this + VEC_OFF + 4) as *mut u32).write_unaligned(f1);
        ((this + VEC_OFF + 8) as *mut u32).write_unaligned(f2);
        let flag = ((this + FLAG_OFF) as *const u8).read();
        let new_flag = (flag & 0xfd) | 1;
        ((this + FLAG_OFF) as *mut u8).write(new_flag);
        new_flag as u32
    }
});
