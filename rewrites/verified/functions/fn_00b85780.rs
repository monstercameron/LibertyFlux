// original: 0x00B85780 row_insert_searched
/// Invoke callee 1 on the row holding `val`, if any.
///
/// Scans the 26 row slots (key at `+0x54`, `STRIDE` bytes apart); when a
/// slot holds `val`, callee 1 receives that row with `(p1..p7)`. No
/// match: nothing happens.
///
/// Original: 0x00B85780 (thiscall, eight stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B85780(this: u32, val: u32, p1: u32, p2: u32, p3: u32, p4: u32, p5: u32, p6: u32, p7: u32) -> u32 {
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
        const SLOTS: u32 = 0x1A;
        const STRIDE: u32 = 0x58;
        const KEY: u32 = 0x54;
        let mut i = 0u32;
        let mut at = 0u32;
        let mut hit = false;
        while i < SLOTS {
            if rd32(this.wrapping_add(KEY).wrapping_add(i.wrapping_mul(STRIDE))) == val {
                at = i;
                hit = true;
                break;
            }
            i += 1;
        }
        if !hit {
            return 0;
        }
        let obj = this.wrapping_add(at.wrapping_mul(STRIDE));
        if obj == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(1, u32, obj, p1, p2, p3, p4, p5, p6, p7);
        0
    }
});
