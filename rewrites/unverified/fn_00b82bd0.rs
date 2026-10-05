// original: 0x00B82BD0 triple_array_init
/// Initialise a table of 150 twelve-byte rows.
///
/// Each row gets `(0, -1, 0)`: a zero key, an invalid marker and a zero
/// value. Rows start at `this` and are `STRIDE` bytes apart.
///
/// Original: 0x00B82BD0 (thiscall, no stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B82BD0(this: u32) -> u32 {
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
        let mut left = ROWS;
        while left != 0 {
            wr32(row, 0);
            wr32(row + 4, 0xFFFF_FFFF);
            wr32(row + 8, 0);
            row = row.wrapping_add(STRIDE);
            left -= 1;
        }
        0
    }
});
