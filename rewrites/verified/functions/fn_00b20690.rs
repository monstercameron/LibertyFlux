// original: 0x00b20690 init_lane_fields (proposed)

/// Initialises a fixed set of fields on the object and returns it.
///
/// Thiscall: keeps only the top bit of the byte at +0x4C, writes 0x3F to
/// the words at +0x40 and +0x44, -1 to the four words at +0x50..+0x5C and
/// 0xFFFFFF to the word at +0x48. Returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00b20690(this: u32) -> u32 {
    unsafe {
        const KEEP_BIT: u8 = 0x80;
        const SMALL: u32 = 0x3f;
        const WHITE24: u32 = 0xffffff;
        let slot = this as *mut u8;
        slot.add(0x4c).write(slot.add(0x4c).read() & KEEP_BIT);
        (this as *mut u32).add(0x44 / 4).write_unaligned(SMALL);
        (this as *mut u32).add(0x40 / 4).write_unaligned(SMALL);
        for off in [0x50u32, 0x54, 0x58, 0x5c] {
            (this as *mut u32).add((off / 4) as usize).write_unaligned(0xffff_ffff);
        }
        (this as *mut u32).add(0x48 / 4).write_unaligned(WHITE24);
    }
    this
});
