// original: 0x00ae4560 aabb_corners_homogeneous (proposed)

/// Expand two opposite corners of an axis-aligned box into its eight corners.
///
/// `a` and `b` each point to three consecutive 32-bit floats (x, y, z); `dst`
/// points to room for eight 16-byte rows. Row `i` (0-7) takes component `k`
/// (0 = x, 1 = y, 2 = z) from `b` when bit `k` of `i` is set, otherwise from
/// `a`, and its fourth word is always 1.0 (`0x3F800000`). All six input words
/// are read before anything is written, and every value is copied bitwise, so
/// NaN payloads survive unchanged.
///
/// Original: 0x00ae4560 (stdcall, three stack words). It never touches `eax`
/// after entry, so it returns its first argument unchanged; the rewrite does
/// the same.
lf_checker_rt::export!(stdcall, rw_00ae4560(a: u32, b: u32, dst: u32) -> u32 {
    unsafe {
        const ONE: u32 = 0x3f80_0000;
        #[inline(always)]
        unsafe fn rd(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(addr: u32, v: u32) {
            unsafe { (addr as *mut u32).write_unaligned(v) }
        }
        let av = [rd(a), rd(a + 4), rd(a + 8)];
        let bv = [rd(b), rd(b + 4), rd(b + 8)];
        let mut i = 0u32;
        while i < 8 {
            let row = dst + i * 16;
            let mut k = 0u32;
            while k < 3 {
                wr(row + k * 4, if (i >> k) & 1 == 1 { bv[k as usize] } else { av[k as usize] });
                k += 1;
            }
            wr(row + 12, ONE);
            i += 1;
        }
        a
    }
});
