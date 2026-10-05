// original: 0x00B82C30 slot_init
/// Initialise one flag slot: kind 0, empty key, default radius.
///
/// Writes kind 0 at `+0`, zeroes `+0x24`/`+0x28`, installs key -1 at
/// `+0x18`, flag 1 at `+0x1A`, the default radius bits at `+0x1C` and kind
/// 0 again at `+0x20`. Returns 0.
///
/// Original: 0x00B82C30 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00B82C30(this: u32) -> u32 {
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
        wr8(this, 0);
        wr32(this + 0x24, 0);
        wr32(this + 0x28, 0);
        unsafe { ((this + 0x18) as *mut u16).write_unaligned(0xFFFF) }
        wr8(this + 0x1A, 1);
        wr32(this + 0x1C, 0x41C8_0000);
        wr8(this + 0x20, 0);
        0
    }
});
