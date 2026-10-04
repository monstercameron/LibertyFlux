// original: 0x00adeaf0 partition (proposed)

/// Partition a word range around a pivot value.
///
/// `first` and `last` bound the array, `pivot` is the pivot value,
/// `comp` a `cdecl(a, b) -> bool` comparator. The low end advances
/// while `comp(element, pivot)` holds; the high end retreats at least
/// once and then while `comp(pivot, element)` holds; when the ends
/// have not crossed, the two elements swap and the scan resumes. The
/// final low end (the cut point) is returned.
///
/// Edge cases: the high end always retreats at least once per pass;
/// the ends are compared unsigned; equal elements may swap without
/// changing memory.
///
/// Original: cdecl, four stack words, one indirect comparator callee
/// (id 1, cdecl, two arguments), no direct callees.
lf_checker_rt::export!(cdecl, rw_00adeaf0(first: u32, last: u32, pivot: u32, comp: u32) -> u32 {
    unsafe {
        let cmp: extern "cdecl" fn(u32, u32) -> u8 =
            core::mem::transmute(comp as usize);
        let rd = |a: u32| (a as *const u32).read_unaligned();
        let wr = |a: u32, v: u32| (a as *mut u32).write_unaligned(v);
        let mut lo = first;
        let mut hi = last;
        loop {
            while cmp(rd(lo), pivot) != 0 {
                lo = lo.wrapping_add(4);
            }
            loop {
                hi = hi.wrapping_sub(4);
                if cmp(pivot, rd(hi)) == 0 {
                    break;
                }
            }
            if lo >= hi {
                return lo;
            }
            let a = rd(lo);
            let b = rd(hi);
            wr(lo, b);
            wr(hi, a);
            lo = lo.wrapping_add(4);
        }
    }
});
