// original: 0x00ca1ed0 slots_zero_15c

/// Clear the five event-source slots at `+0x15c`.
///
/// Zeroes five consecutive words starting at `+0x15c`. No value is returned.
///
/// Original: 0x00ca1ed0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00ca1ed0(this: u32) -> u32 {
    #[inline(always)]
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe {
        const SLOTS: u32 = 0x15c;
        const COUNT: u32 = 5;
        let mut i = 0u32;
        while i < COUNT {
            wr32(this + SLOTS + i * 4, 0);
            i += 1;
        }
        0
    }
});
