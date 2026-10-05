// original: 0x00AED160 kv_lower_bound (proposed)

/// First 8-byte entry whose key is not below the target (lower bound).
///
/// `first` and `last` delimit an array of 8-byte entries (key at offset 0,
/// payload at offset 4, payload never read); `keyptr` points at the target
/// key word. Returns the address of the first entry with key >= target, or
/// `last` when every entry is below it (`first` when the range is empty).
/// Standard binary search: `n = (last - first) / 8` unsigned bytes shifted
/// arithmetically, halve `n` each step, compare unsigned. The range is
/// assumed sorted by key; on unsorted input the result is whatever the
/// search walk finds, exactly as the original walks it.
///
/// Original: 0x00AED160 (cdecl, three stack words, no calls).
lf_checker_rt::export!(cdecl, rw_00aed160(first: u32, last: u32, keyptr: u32) -> u32 {
    unsafe {
        const ENTRY: u32 = 8;
        const KEY_OFF: u32 = 0;
        let target = ((keyptr.wrapping_add(KEY_OFF)) as *const u32).read_unaligned();
        let mut base = first;
        // `sub` then arithmetic `sar`: replicate with a wrapping subtract
        // cast to signed before the shift, so a wrapped range also matches.
        let mut n = (last.wrapping_sub(base) as i32) >> 3;
        if n <= 0 {
            return base;
        }
        loop {
            let half = (n as u32) >> 1;
            let mid = base.wrapping_add(half.wrapping_mul(ENTRY));
            if ((mid as *const u32).read_unaligned()) >= target {
                n = half as i32;
            } else {
                base = mid.wrapping_add(ENTRY);
                n = n - half as i32 - 1;
            }
            if n <= 0 {
                break;
            }
        }
        base
    }
});
