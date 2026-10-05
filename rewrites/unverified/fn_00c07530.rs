// original: 0x00c07530 stream_pair_forward_reset
/// Forward this pair's fields to the helper, then clear the pair.
///
/// Calls the pair helper (thiscall/2) with `this`+0 and the 16-bit word at
/// `this`+6, then zeroes +0 and +4 and the flag byte at +8. Returns 0.
/// Thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c07530(this: u32) -> u32 {
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

        const HELPER: u32 = 1;
        let _: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, this, rd32(this), rd16(this + 6));
        wr32(this, 0);
        wr32(this + 4, 0);
        wr8(this + 8, 0);
        0
    }
});
