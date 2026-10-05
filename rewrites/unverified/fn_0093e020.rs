// original: 0x0093e020 heap_build (proposed)

/// Heapify the pointer range [`base`, `end`) bottom-up via the heap callee.
///
/// With count `(end - base) / 4` (signed), ranges shorter than 2 are
/// already heaps and make no calls. Otherwise calls the heap callee with
/// (`base`, i, count, `base[i]`, `extra`) for i from `(count - 2) / 2`
/// down to 0. The return value is the last callee answer, or entry
/// garbage for a short range, so it is not compared.
///
/// Original: 0x0093e020 (cdecl, three stack words; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093e020(base: u32, end: u32, extra: u32) -> u32 {
    const HEAP_CALLEE: u32 = 1;
    unsafe {
        let count = ((end.wrapping_sub(base)) as i32 >> 2) as u32;
        if (count as i32) < 2 {
            return 0;
        }
        let mut i = (((count as i32) - 2) / 2) as u32;
        loop {
            let v = (base.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(HEAP_CALLEE, u32, base, i, count, v, extra);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        0
    }
});
