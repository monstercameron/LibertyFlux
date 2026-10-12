// original: 0x00876C00 reset_frame_sample_bounds

/// Reset the frame-sample bounds record.
///
/// The record starts at `this`. It clears the six pointer and state words
/// through `+0x14`, writes the largest finite f32 sentinel to the three
/// upper-bound words at `+0x20..+0x28`, writes the small positive bound
/// sentinel to `+0x30..+0x38`, and clears the final word at `+0x40`.
/// It returns the original record pointer.
///
/// Original calling convention: thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00876c00(this: u32) -> u32 {
    const ZERO_WORDS: [u32; 6] = [0x00, 0x04, 0x08, 0x0c, 0x10, 0x14];
    const MAX_BOUND: u32 = 0x7f7f_ffff;
    const MIN_BOUND: u32 = 0x0080_0000;
    unsafe {
        for offset in ZERO_WORDS {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(0x20) as *mut u32).write_unaligned(MAX_BOUND);
        (this.wrapping_add(0x24) as *mut u32).write_unaligned(MAX_BOUND);
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(MAX_BOUND);
        (this.wrapping_add(0x30) as *mut u32).write_unaligned(MIN_BOUND);
        (this.wrapping_add(0x34) as *mut u32).write_unaligned(MIN_BOUND);
        (this.wrapping_add(0x38) as *mut u32).write_unaligned(MIN_BOUND);
        (this.wrapping_add(0x40) as *mut u32).write_unaligned(0);
    }
    this
});
