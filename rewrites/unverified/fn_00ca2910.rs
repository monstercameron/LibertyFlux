// original: 0x00ca2910 copy16_set_flag

/// Copy a 16-byte face parameter block and mark it present.
///
/// Copies 16 bytes from `src` to `this` (two 8-byte moves) and sets byte
/// `+0x24` to 1. No value is returned.
///
/// Original: 0x00ca2910 (thiscall, one stack word = src pointer).
lf_checker_rt::export!(thiscall, rw_00ca2910(this: u32, src: u32) -> u32 {
    #[inline(always)]
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn wr8(a: u32, v: u8) {
        unsafe { (a as *mut u8).write(v) }
    }
    unsafe {
        const PRESENT: u32 = 0x24;
        let mut i = 0u32;
        while i < 16 {
            wr8(this + i, rd8(src + i));
            i += 1;
        }
        wr8(this + PRESENT, 1);
        0
    }
});
