// original: 0x009B6EE0 input_first_live_slot_index (proposed)

/// Finds the first of ten slots whose answer word is nonzero, polling the slot callee with indices 0 to 9. Returns the index of the first slot whose word at +0x80 is nonzero, or -1 when all ten read zero. The original bounds the loop with a signed compare; the indices stay non-negative so the bound is exact.
///
/// Original: 0x009B6EE0 (thiscall).
lf_checker_rt::export!(thiscall, rw_009B6EE0(this: u32) -> u32 {
    const WORD_OFFSET: u32 = 0x80;
    const SLOT_COUNT: i32 = 10;

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn write_byte(address: u32, value: u8) {
        unsafe { (address as *mut u8).write(value) }
    }

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    unsafe {
        let mut index: i32 = 0;
        loop {
            let answer = lf_checker_rt::callee_thiscall!(1, u32, this, index as u32);
            if read_word(answer + WORD_OFFSET) != 0 {
                return index as u32;
            }
            index += 1;
            if index >= SLOT_COUNT {
                return 0xFFFF_FFFF;
            }
        }
    }
});
