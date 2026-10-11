// original: 0x00C30C40 vehicle_heading_clear_state

/// Clear the seven 32-bit state words in the heading record at byte offsets
/// `0x00` through `0x18`, inclusive, leaving later bytes untouched. The
/// original is a thiscall method with no stack arguments and returns zero in
/// EAX.
lf_checker_rt::export!(thiscall, rw_00c30c40(this: *mut u8) -> u32 {
    const ZERO_BITS: u32 = 0;
    unsafe {
        for offset in [0x00usize, 0x04, 0x08, 0x0c, 0x10, 0x14, 0x18] {
            this.add(offset).cast::<u32>().write_unaligned(ZERO_BITS);
        }
    }
    ZERO_BITS
});
