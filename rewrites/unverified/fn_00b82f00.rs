// original: 0x00B82F00 triple_array_remove
/// Remove every triple row matching `(k0, k1, k2)`.
///
/// Scans the 150 twelve-byte rows; a row whose words equal `k0`, `k1`
/// and `k2` (in slot order 0, 1, 2) is reset to `(0, -1, 0)`. When the
/// third key is 1 or 2 and the row key is nonzero, callee 1 first observes
/// the doomed row.
///
/// Original: 0x00B82F00 (thiscall, three stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B82F00(this: u32, k0: u32, k2: u32, k1: u32) -> u32 {
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
        const ROWS: u32 = 0x96;
        const STRIDE: u32 = 0x0C;
        let mut row = this;
        let mut i = 0u32;
        while i < ROWS {
            if rd32(row) == k0 && rd32(row + 4) == k1 && rd32(row + 8) == k2 {
                if k2.wrapping_sub(1) <= 1 && rd32(row) != 0 {
                    lf_checker_rt::callee_stdcall!(1, u32, row);
                }
                wr32(row, 0);
                wr32(row + 4, 0xFFFF_FFFF);
                wr32(row + 8, 0);
            }
            row = row.wrapping_add(STRIDE);
            i += 1;
        }
        0
    }
});
