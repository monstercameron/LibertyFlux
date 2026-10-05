// original: 0x00c06780 stream_view_copy
/// Copy the view `src` into `this` through the parts initialiser.
///
/// Does nothing and returns `this` when `src == this`. Otherwise forwards
/// the source fields to the parts initialiser (thiscall/8): `src`+4, `src`+8
/// (floats), `src`+0x110, `src`+0x114, `src`+0xc, `src`+0x10c (float),
/// `src`+0, `src`+0x118, and returns `this`. Thiscall: one stack word,
/// callee cleans 4.
lf_checker_rt::export!(thiscall, rw_00c06780(this: u32, src: u32) -> u32 {
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
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const INIT: u32 = 1;
        if this == src {
            return this;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(INIT, u32, this, rd32(src + 4), rd32(src + 8),
            rd32(src + 0x110), rd32(src + 0x114), rd32(src + 0x0c), rd32(src + 0x10c),
            rd32(src), rd32(src + 0x118));
        this
    }
});
