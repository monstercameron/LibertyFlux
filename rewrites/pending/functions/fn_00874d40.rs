// original: 0x00874d40 blend_zero_mid
/// Clear the middle weight block of a blend/add motion request.
///
/// Zeroes the thirty-two dwords at offsets `0x94..=0x110` of the object
/// and returns zero.
export!(thiscall, rw_00874d40(this: u32) -> u32 {
    unsafe {
        const FIRST_WORD: usize = 0x94 / 4;
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
