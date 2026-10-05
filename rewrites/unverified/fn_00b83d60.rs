// original: 0x00B83D60 all_c8_check
/// Report whether any of the six marker words differs from 0xC8.
///
/// Returns 0 when all six words at `this` hold `MARK`, 1 on the first
/// word that differs.
///
/// Original: 0x00B83D60 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00B83D60(this: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const MARK: u32 = 0xC8;
        let mut i = 0u32;
        while i < 6 {
            if rd32(this + i * 4) != MARK {
                return 1;
            }
            i += 1;
        }
        0
    }
});
