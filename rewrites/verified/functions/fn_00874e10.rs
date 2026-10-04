// original: 0x00874e10 blend_zero_hi
/// Clear the upper weight block of a blend/add motion request.
///
/// Zeroes the thirty-two dwords at offsets `0x114..=0x190` of the object
/// and returns zero.
export!(thiscall, rw_00874e10(this: u32) -> u32 {
    unsafe {
        const FIRST_WORD: usize = 0x114 / 4;
        const COUNT: usize = 32;
        let base = this as *mut u32;
        let mut i: usize = 0;
        while i < COUNT {
            base.add(FIRST_WORD + i).write(0);
            i += 1;
        }
        0
    }
});
