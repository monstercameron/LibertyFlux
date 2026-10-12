// original: 0x009B9490 input_clear_stale_and_sync (proposed)

/// Clears the receiver's stale flag and runs the sync callee. Writes 0 to the flag byte at +0x530, calls the sync helper on the receiver and returns 1 in AL.
///
/// Original: 0x009B9490 (thiscall).
lf_checker_rt::export!(thiscall, rw_009B9490(this: u32) -> u32 {
    const FLAG_OFFSET: u32 = 0x530;

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
        write_byte(this + FLAG_OFFSET, 0);
        lf_checker_rt::callee_thiscall!(1, u32, this);
        1
    }
});
