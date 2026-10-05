// original: 0x00a4f0a0 sort_partition (proposed)

/// Partition [first, last) around the pivot key, returning the split point.
///
/// The pivot key comes from the `pivot` element (float at +0x80). The low
/// scan advances while the pivot key is strictly greater than each slot's;
/// the high scan retreats while each slot's key is strictly greater than
/// the pivot's. Equal or unordered (NaN) keys stop either scan. While the
/// scans have not crossed (unsigned compare), the two slots are swapped and
/// the low scan steps past. The fourth argument is unused. Cdecl, four
/// stack words, split point in eax.
lf_checker_rt::export!(cdecl, rw_00a4f0a0(first: u32, last: u32, pivot: u32, _extra: u32) -> u32 {
    unsafe {
        const KEY_OFF: u32 = 0x80;
        unsafe fn rd(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        unsafe fn key(elem: u32) -> f32 {
            unsafe { f32::from_bits(rd(elem.wrapping_add(KEY_OFF))) }
        }
        let pk = key(pivot);
        let mut lo = first;
        let mut hi = last;
        loop {
            if pk > key(rd(lo)) {
                loop {
                    lo = lo.wrapping_add(4);
                    if !(pk > key(rd(lo))) {
                        break;
                    }
                }
            }
            let mut x = rd(hi.wrapping_sub(4));
            hi = hi.wrapping_sub(4);
            if key(x) > pk {
                loop {
                    x = rd(hi.wrapping_sub(4));
                    hi = hi.wrapping_sub(4);
                    if !(key(x) > pk) {
                        break;
                    }
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
