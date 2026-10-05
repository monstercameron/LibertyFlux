// original: 0x00c06650 stream_pair_reset
/// Reset a streaming pair object around a helper call.
///
/// Zeroes the words at +0 and +4, calls the pair helper (thiscall/2) with two
/// zero arguments, then zeroes +0 and +4 again plus the flag byte at +8, and
/// returns `this`. Thiscall with no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c06650(this: u32) -> u32 {
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
        wr32(this, 0);
        wr32(this + 4, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(HELPER, u32, this, 0, 0);
        wr32(this, 0);
        wr32(this + 4, 0);
        wr8(this + 8, 0);
        this
    }
});
