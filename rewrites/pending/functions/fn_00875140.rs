// original: 0x00875140 rage::crmtRequestSource<202>::vf1
/// Reset a motion-source object to the empty state.
///
/// Zeroes the thirty-two dwords at offsets `0x14..=0x90` of the object
/// and returns zero.
export!(thiscall, rw_00875140(this: u32) -> u32 {
    unsafe {
        const FIRST_WORD: usize = 0x14 / 4;
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
