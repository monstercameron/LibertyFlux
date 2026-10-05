// original: 0x00B83C10 row_copy_6
/// Copy one six-key row struct from `src` to `this`.
///
/// Copies the header word at `+0x54`, then per key: the key word, four
/// value bytes at `+0x18`, two flag bytes at `+0x30` and the mask word at
/// `+0x3C` (written twelve times, same value).
///
/// Original: 0x00B83C10 (thiscall, one stack argument, no return value).
lf_checker_rt::export!(thiscall, rw_00B83C10(this: u32, src: u32) -> u32 {
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
        wr32(this + 0x54, rd32(src + 0x54));
        let mut k = 0u32;
        while k < 6 {
            wr32(this + k * 4, rd32(src + k * 4));
            let mut b = 0u32;
            while b < 4 {
                wr8(this + 0x18 + k * 4 + b, rd8(src + 0x18 + k * 4 + b));
                b += 1;
            }
            wr8(this + 0x30 + k * 2, rd8(src + 0x30 + k * 2));
            wr8(this + 0x31 + k * 2, rd8(src + 0x31 + k * 2));
            let mut r = 0u32;
            while r < 12 {
                wr32(this + 0x3C + k * 4, rd32(src + 0x3C + k * 4));
                r += 1;
            }
            k += 1;
        }
        0
    }
});
