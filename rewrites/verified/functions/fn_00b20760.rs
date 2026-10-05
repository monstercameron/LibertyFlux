// original: 0x00b20760 init_head_fields (proposed)

/// Zeroes the object's head fields, presets three, returns the object.
///
/// Thiscall: zeroes the six words at +0x00..+0x14 and the halfword at
/// +0x50, writes 0x3F to the word at +0x44 and zeroes the words at +0x40,
/// +0x48 and +0x4C. Returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00b20760(this: u32) -> u32 {
    unsafe {
        const SMALL: u32 = 0x3f;
        for off in [0x00u32, 0x04, 0x08, 0x0c, 0x10, 0x14] {
            (this as *mut u32).add((off / 4) as usize).write_unaligned(0);
        }
        ((this + 0x50) as *mut u16).write_unaligned(0);
        (this as *mut u32).add(0x44 / 4).write_unaligned(SMALL);
        for off in [0x40u32, 0x48, 0x4c] {
            (this as *mut u32).add((off / 4) as usize).write_unaligned(0);
        }
    }
    this
});
