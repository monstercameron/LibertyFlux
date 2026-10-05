// original: 0x00c06480 stream_read_counted_span
/// Read a counted span through the reader helper twice.
///
/// Calls the reader (cdecl/3) with (`arg`, `this`+0x4c, 4); when the word it
/// filled is non-zero, calls the reader again with (`arg`, `this`+0x48, that
/// word). Returns nothing meaningful. Thiscall: one stack word, callee
/// cleans 4. The two call sites share one helper but use separate callee ids
/// so the first call's fill is observed without writing through the second
/// call's value arguments.
lf_checker_rt::export!(thiscall, rw_00c06480(this: u32, arg: u32) -> u32 {
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

        const READ1: u32 = 1;
        const READ2: u32 = 2;
        let _: u32 = lf_checker_rt::callee_cdecl!(READ1, u32, arg, this + 0x4c, 4);
        let n = rd32(this + 0x4c);
        if n != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(READ2, u32, arg, rd32(this + 0x48), n);
        }
        0
    }
});
