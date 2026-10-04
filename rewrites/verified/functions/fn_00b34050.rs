// original: 0x00b34050 stable_front_insert (proposed)

/// Sweep an array of 16-byte keyed elements, fronting smaller keys.
///
/// `first` and `last` bound the array (a whole number of 16-byte elements;
/// `gap` is unread); each element's first word is a float key. An empty or
/// single-element range returns `last`. Otherwise every element after the
/// first is visited in order: when the first element's key is strictly
/// greater than the visitor's (an unordered pair counts as not greater), the
/// visitor's element moves to the front and the elements before it shift up
/// one slot; otherwise a visit callee takes the visitor's address, its four
/// words and `extra`. Returns `last` for a short range, else the last
/// callee answer, or `first + 16` when the last visit fronted.
///
/// Original: 0x00b34050 (cdecl, four stack words; one six-word callee).
lf_checker_rt::export!(cdecl, rw_00b34050(first: u32, last: u32, gap: u32, extra: u32) -> u32 {
    unsafe {
        const ELEM: u32 = 16;
        const VISIT: u32 = 1;
        #[inline(always)]
        unsafe fn key(p: u32) -> f32 {
            unsafe { f32::from_bits((p as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn copy16(dst: u32, src: u32) {
            unsafe {
                for off in [0usize, 8] {
                    ((dst as *mut u8).byte_add(off) as *mut u64).write_unaligned(
                        ((src as *const u8).byte_add(off) as *const u64).read_unaligned(),
                    );
                }
            }
        }
        let _ = gap;
        if first == last {
            return last;
        }
        let mut cur = first.wrapping_add(ELEM);
        if cur == last {
            return last;
        }
        let mut answer = last;
        while cur != last {
            if key(first) > key(cur) {
                let mut tmp = [0u64; 2];
                tmp[0] = (cur as *const u64).read_unaligned();
                tmp[1] = ((cur as *const u8).byte_add(8) as *const u64).read_unaligned();
                let mut p = cur;
                while p != first {
                    copy16(p, p.wrapping_sub(ELEM));
                    p = p.wrapping_sub(ELEM);
                }
                (first as *mut u64).write_unaligned(tmp[0]);
                ((first as *mut u8).byte_add(8) as *mut u64).write_unaligned(tmp[1]);
                answer = first.wrapping_add(ELEM);
            } else {
                let b0 = (cur as *const u32).read_unaligned();
                let b1 = (cur as *const u32).byte_add(4).read_unaligned();
                let b2 = (cur as *const u32).byte_add(8).read_unaligned();
                let b3 = (cur as *const u32).byte_add(12).read_unaligned();
                answer = lf_checker_rt::callee_cdecl!(VISIT, u32, cur, b0, b1, b2, b3, extra);
            }
            cur = cur.wrapping_add(ELEM);
        }
        answer
    }
});
