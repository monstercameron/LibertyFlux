// original: 0x00c05d10 stream_header_init
/// Initialise a streaming header object with constant defaults.
///
/// Writes 0x6408 at +0 (word), 0x32 at +2, 0 at +4, 0xFF000000 at +8, 0 at
/// +0xc, -1 at +0x10, 0 at +0x14 and +0x18, -1 at +0x1c, 4 at +0x20 and
/// +0x24, 0 at +0x28, +0x2c, +0x30 and +0x48. Bit 2 of the byte at +3 ends up
/// holding bit 0 of the mode global (the original reaches that with two
/// read-modify-write steps that clear every other bit). Returns `this`.
/// Thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c05d10(this: u32) -> u32 {
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

        const MODE_GLOBAL: u32 = 0x01176D38;
        wr8(this + 3, 0);
        unsafe { ((this) as *mut u16).write_unaligned(0x6408) };
        wr8(this + 2, 0x32);
        wr32(this + 4, 0);
        wr32(this + 8, 0xFF00_0000);
        wr32(this + 0x0c, 0);
        wr32(this + 0x10, 0xFFFF_FFFF);
        wr32(this + 0x14, 0);
        wr32(this + 0x18, 0);
        wr32(this + 0x1c, 0xFFFF_FFFF);
        wr32(this + 0x20, 4);
        wr32(this + 0x24, 4);
        wr32(this + 0x28, 0);
        wr32(this + 0x2c, 0);
        wr32(this + 0x30, 0);
        wr32(this + 0x48, 0);
        let mode = rd8(lf_checker_rt::relocated(MODE_GLOBAL));
        wr8(this + 3, (mode & 1) << 2);
        this
    }
});
