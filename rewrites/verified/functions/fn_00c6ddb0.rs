// original: 0x00c6ddb0 partial_sort_range
/// Partial sort: heap-select the smallest elements of `[mid, last)` into the
/// heap over `[first, mid)`, then sort that heap in place. The fourth
/// argument is unused.
export!(cdecl, rw_00c6ddb0(first: u32, mid: u32, last: u32, _u: u32, extra: u32) -> () {
    unsafe {
        callee_cdecl!(1, u32, first, mid, extra);
        let mut p = mid;
        if mid < last {
            loop {
                if (*(p as *const u32) as i32) < (*(first as *const u32) as i32) {
                    let k = *(p as *const u32);
                    let v = *((p + 4) as *const u32);
                    *(p as *mut u32) = *(first as *const u32);
                    *((p + 4) as *mut u32) = *((first + 4) as *const u32);
                    let count = ((mid.wrapping_sub(first) as i32) >> 3) as u32;
                    callee_cdecl!(2, u32, first, 0, count, k, v, extra);
                }
                p = p.wrapping_add(8);
                if p >= last {
                    break;
                }
            }
        }
        let mut m = mid;
        let mut span = mid.wrapping_sub(first);
        while ((span & 0xFFFF_FFF8) as i32) > 8 {
            callee_cdecl!(3, u32, first, m, extra);
            span = span.wrapping_sub(8);
            m = m.wrapping_sub(8);
        }
    }
});
