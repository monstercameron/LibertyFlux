// original: 0x00ade7f0 insertion_sort (proposed)

/// Insertion-sort a word range with a guarded fast path for new minima.
///
/// `first` and `last` bound the array, the third argument is unread,
/// `comp` is a `cdecl(a, b) -> bool` comparator. Ranges shorter than
/// two words return at once. Each later element is tested against the
/// first: when `comp(value, *first)` holds, the block is shifted up by
/// one word through the move callee (id 2: destination, source, byte
/// count) and the value becomes the new first element; otherwise the
/// unguarded-insert callee (id 3: position, value, comparator) places
/// it, which is safe because the first element now stops the scan.
///
/// Edge cases: empty and single-element ranges make no calls; equal
/// elements keep their order through the insert callee.
///
/// Original: cdecl, four stack words, one indirect comparator callee
/// (id 1, cdecl, two arguments) and two direct callees (id 2 cdecl
/// with three arguments, id 3 cdecl with three). Returns nothing.
lf_checker_rt::export!(cdecl, rw_00ade7f0(first: u32, last: u32, _unused: u32, comp: u32) -> u32 {
    unsafe {
        let cmp: extern "cdecl" fn(u32, u32) -> u8 =
            core::mem::transmute(comp as usize);
        let rd = |a: u32| (a as *const u32).read_unaligned();
        let wr = |a: u32, v: u32| (a as *mut u32).write_unaligned(v);
        if first == last {
            return 0;
        }
        let mut cur = first.wrapping_add(4);
        if cur == last {
            return 0;
        }
        while cur != last {
            let val = rd(cur);
            if cmp(val, rd(first)) != 0 {
                let n = cur.wrapping_sub(first);
                if (n as i32) > 0 {
                    lf_checker_rt::callee_cdecl!(2, u32, cur.wrapping_sub(n).wrapping_add(4), first, n);
                }
                wr(first, val);
            } else {
                lf_checker_rt::callee_cdecl!(3, u32, cur, val, comp);
            }
            cur = cur.wrapping_add(4);
        }
    }
    0
});
