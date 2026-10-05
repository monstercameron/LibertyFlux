// original: 0x00B83D80 forward_4
/// Forward to callee 1 with the object's inner value first.
///
/// Loads the inner pointer at `+0x224`, reads its word at `+0xD0` and
/// invokes callee 1 with `(inner, obj, p1, p2)`, returning its answer.
///
/// Original: 0x00B83D80 (stdcall, three stack arguments).
lf_checker_rt::export!(stdcall, rw_00B83D80(obj: u32, p1: u32, p2: u32) -> u32 {
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
        let inner = rd32(obj + 0x224);
        let v = rd32(inner + 0xD0);
        lf_checker_rt::callee_stdcall!(1, u32, v, obj, p1, p2)
    }
});
