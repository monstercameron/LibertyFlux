// original: 0x00AC5AD0 transpose_4rows_of_3 (proposed)

/// Transpose four 3-float rows into three 4-float columns.
///
/// Rows `a`, `b`, `c`, `d` each hold three floats at offsets 0, 4, 8. The
/// original reads all twelve first, then writes column 0 `(a0, b0, c0, d0)`
/// to `o1`, column 1 to `o2` and column 2 to `o3` (cdecl, seven pointers).
/// Reading everything before writing keeps overlapping buffers identical.
lf_checker_rt::export!(cdecl, rw_00AC5AD0(a: u32, b: u32, c: u32, d: u32, o1: u32, o2: u32, o3: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        unsafe {
            let (a0, a1, a2) = (rd(a), rd(a + 4), rd(a + 8));
            let (b0, b1, b2) = (rd(b), rd(b + 4), rd(b + 8));
            let (c0, c1, c2) = (rd(c), rd(c + 4), rd(c + 8));
            let (d0, d1, d2) = (rd(d), rd(d + 4), rd(d + 8));
            wr(o1, a0);
            wr(o1 + 4, b0);
            wr(o1 + 8, c0);
            wr(o1 + 12, d0);
            wr(o2, a1);
            wr(o2 + 4, b1);
            wr(o2 + 8, c1);
            wr(o2 + 12, d1);
            wr(o3, a2);
            wr(o3 + 4, b2);
            wr(o3 + 8, c2);
            wr(o3 + 12, d2);
            0
        }
    }
});
