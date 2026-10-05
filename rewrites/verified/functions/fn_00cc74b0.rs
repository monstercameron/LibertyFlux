// original: 0x00CC74B0 euphoria_zero_block (proposed)

/// Clear the state words of a feedback sub-object.
///
/// Writes zero to nine scattered words of the object at `this` (offsets
/// `0x00`, `0x10`, `0x14`, `0x18`, `0x20`, `0x24`, `0x28`, `0x30`, `0x38`)
/// and returns `this`. Every other word, including the gaps at `0x04`-`0x0c`,
/// `0x1c`, `0x2c` and `0x34`-`0x36`, is left untouched.
///
/// Original: 0x00CC74B0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00cc74b0(this: u32) -> u32 {
    unsafe {
        const SLOTS: [u32; 9] = [0x00, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x38];
        for off in SLOTS {
            (this.wrapping_add(off) as *mut u32).write_unaligned(0);
        }
        this
    }
});
