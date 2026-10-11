// original: 0x00CC3130 CPedMoveBlendOnFoot::vf12
/// Set the blend-rate word to 3.0. When bit one at receiver offset 0x50 is
/// set, write 2.0 to offsets 0x1C and 0x20; otherwise write 3.0 and 0. The
/// return value is the receiver flag word shifted right one bit.
lf_checker_rt::export!(thiscall, rw_00cc3130(this: *mut u8) -> u32 {
    const FLAGS_OFFSET: usize = 0x50;
    const WEIGHT_A_OFFSET: usize = 0x1C;
    const WEIGHT_B_OFFSET: usize = 0x20;
    const BLEND_RATE_OFFSET: usize = 0x54;
    const TWO: u32 = 0x40000000;
    const THREE: u32 = 0x40400000;
    let shifted_flags = unsafe { this.add(FLAGS_OFFSET).cast::<u32>().read_unaligned() >> 1 };
    unsafe { this.add(BLEND_RATE_OFFSET).cast::<u32>().write_unaligned(THREE) };
    if shifted_flags & 1 != 0 {
        unsafe {
            this.add(WEIGHT_A_OFFSET).cast::<u32>().write_unaligned(TWO);
            this.add(WEIGHT_B_OFFSET).cast::<u32>().write_unaligned(TWO);
        }
    } else {
        unsafe {
            this.add(WEIGHT_A_OFFSET).cast::<u32>().write_unaligned(THREE);
            this.add(WEIGHT_B_OFFSET).cast::<u32>().write_unaligned(0);
        }
    }
    shifted_flags
});
