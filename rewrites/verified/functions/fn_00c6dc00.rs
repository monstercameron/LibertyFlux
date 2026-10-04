// original: 0x00c6dc00 insertion_sort_range
/// Insertion-sort `[first, last)`: elements smaller than the front element
/// rotate the whole prefix right, the rest go through the unguarded helper.
/// The third argument is unused, the fourth is forwarded untouched.
export!(cdecl, rw_00c6dc00(first: u32, last: u32, _u: u32, extra: u32) -> () {
    unsafe {
        if first == last {
            return;
        }
        let mut cur = first.wrapping_add(8);
        if cur == last {
            return;
        }
        let mut span: u32 = 8;
        loop {
            let key = *(cur as *const u32);
            let val = *((cur + 4) as *const u32);
            if (key as i32) < (*(first as *const u32) as i32) {
                let n = (span as i32) >> 3;
                if n > 0 {
                    let mut d = cur;
                    let mut s = cur.wrapping_sub(8);
                    let mut i = n;
                    while i > 0 {
                        *(d as *mut u32) = *(s as *const u32);
                        *((d + 4) as *mut u32) = *((s + 4) as *const u32);
                        d = d.wrapping_sub(8);
                        s = s.wrapping_sub(8);
                        i -= 1;
                    }
                }
                *(first as *mut u32) = key;
                *((first + 4) as *mut u32) = val;
            } else {
                callee_cdecl!(1, u32, cur, key, val, extra);
            }
            cur = cur.wrapping_add(8);
            span = span.wrapping_add(8);
            if cur == last {
                break;
            }
        }
    }
});
