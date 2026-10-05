// original: 0x00B82F60 flag_set_1810
/// Store the low byte of `val` into the enable flag.
///
/// The flag lives at `FLAG` past `this`. Only the low byte is stored.
///
/// Original: 0x00B82F60 (thiscall, one stack argument, no return value).
lf_checker_rt::export!(thiscall, rw_00B82F60(this: u32, val: u32) -> u32 {
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
        const FLAG: u32 = 0x1810;
        wr8(this + FLAG, (val & 0xFF) as u8);
        0
    }
});
