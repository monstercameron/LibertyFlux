// original: 0x00c05cd0 stream_slot_clear
/// Reset a streaming slot object to its empty state.
///
/// Zeroes the words at +0x20, +0x24, +0x2c, +0x28, +0x30, +0x3c, +0x48 and
/// +0x4c, writes -1 to +0x34 and +0x40, clears the flag byte at +0, and
/// returns `this`. Thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c05cd0(this: u32) -> u32 {
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

        const NONE: u32 = 0xFFFF_FFFF;
        for off in [0x20u32, 0x24, 0x2c, 0x28, 0x30, 0x3c, 0x48, 0x4c] {
            wr32(this + off, 0);
        }
        wr32(this + 0x34, NONE);
        wr32(this + 0x40, NONE);
        wr8(this, 0);
        this
    }
});
