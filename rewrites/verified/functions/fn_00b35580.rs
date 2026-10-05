// original: 0x00B35580 unguarded_partition_16

/// Hoare-partition the 16-byte-element run `[first, last)` around `pivot`
/// by the float key at offset 0; returns the split pointer.
///
/// The low scan advances past keys strictly below the pivot, the high scan
/// retreats past keys strictly above it (both `comiss` scans treat an
/// unordered/NaN comparison as "stop"), out-of-place pairs are swapped, and
/// the scans repeat until they meet or cross. The ends are unguarded: the
/// caller keeps a key at or below the pivot at the bottom and a key at or
/// above it at the top. Cdecl, three stack words, returns the split pointer.
///
/// Original: 0x00B35580 (true size 220; the batch list truncates it at 210).

lf_checker_rt::export!(cdecl, rw_00B35580(first: u32, last: u32, pivot: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 16;
        let pv = f32::from_bits(pivot);
        let mut lo = first;
        let mut hi = last;
        loop {
            loop {
                let k = (lo as *const f32).read_unaligned();
                if !(pv > k) {
                    break;
                }
                lo = lo.wrapping_add(STRIDE);
            }
            loop {
                hi = hi.wrapping_sub(STRIDE);
                let k = (hi as *const f32).read_unaligned();
                if !(k > pv) {
                    break;
                }
            }
            if lo >= hi {
                return lo;
            }
            for i in 0..4u32 {
                let a = (lo.wrapping_add(i * 4) as *const u32).read_unaligned();
                let b = (hi.wrapping_add(i * 4) as *const u32).read_unaligned();
                (lo.wrapping_add(i * 4) as *mut u32).write_unaligned(b);
                (hi.wrapping_add(i * 4) as *mut u32).write_unaligned(a);
            }
            lo = lo.wrapping_add(STRIDE);
        }
    }
});
