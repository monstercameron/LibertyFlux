// original: 0x005dd3b0 task_flag_check_store

/// Clear word +0x20 of one object, and set a flag when another holds 0x12b.
///
/// Zeroes the word at `b + 0x20` unconditionally, then sets bit 1 of
/// the byte at `b + 0x14` when the word at `a + 0x0c` equals `0x12b`.
/// Returns `a` (the original leaves it in eax).
///
/// Original: 0x005dd3b0 (cdecl, two pointers).
lf_checker_rt::export!(cdecl, rw_005dd3b0(a: u32, b: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const WANT: u32 = 0x12b;
        const FLAG: u8 = 2;
        wr32(b + 0x20, 0);
        if rd32(a + 0x0c) == WANT {
            let f = rd8(b + 0x14);
            wr8(b + 0x14, f | FLAG);
        }
        a
    }
});
