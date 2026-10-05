// original: 0x00c06680 stream_view_init_defaults
/// Initialise a streaming view object with default values.
///
/// Writes the colour 0xFF000000 at +0, zeroes +4, +8 and the flag byte at
/// +0xc, writes 1.0f at +0x10c, +0x110 and +0x114, writes 7 at +0x118, and
/// returns `this`. Thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c06680(this: u32) -> u32 {
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
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        const OPAQUE_BLACK_RED: u32 = 0xFF00_0000;
        const ONE: u32 = 0x3F80_0000;
        wr32(this, OPAQUE_BLACK_RED);
        wr32(this + 4, 0);
        wr32(this + 8, 0);
        for off in [0x10cu32, 0x110, 0x114] {
            wr32(this + off, ONE);
        }
        wr32(this + 0x118, 7);
        wr8(this + 0x0c, 0);
        this
    }
});
