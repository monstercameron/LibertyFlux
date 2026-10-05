// original: 0x00bf8f60 task_normalize_copy3

/// Run the two-word normaliser over a0 and the +0x1c field, then copy three
/// dwords from +0x20 into the pointed-to block.
///
/// Original: 0x00bf8f60 (thiscall, 2 stack words).
lf_checker_rt::export!(thiscall, rw_00bf8f60(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn r8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn w8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn r32u(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn w32u(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        lf_checker_rt::callee_cdecl!(1, u32, a0, r32u(this.wrapping_add(0x1c)));
        w32u(a1, r32u(this.wrapping_add(0x20)));
        w32u(a1.wrapping_add(4), r32u(this.wrapping_add(0x24)));
        w32u(a1.wrapping_add(8), r32u(this.wrapping_add(0x28)));
        0
    }
});
