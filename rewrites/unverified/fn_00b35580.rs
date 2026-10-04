// original: 0x00b35580 partition_16b_key0 (proposed)

/// Partition the run of 16-byte elements in [`first`, `last`) around the
/// `pivot` key float, comparing leading floats with the ordered greater
/// test: scan forward past elements below the pivot, scan backward past
/// elements above it, swap the pair that stopped both scans, and repeat
/// until the scans meet. An unordered (NaN) comparison stops either scan,
/// matching the original's branch-on-greater. Returns the meeting point.
/// Original: 0x00b35580 (cdecl, three stack words: first, last, pivot bits).
lf_checker_rt::export!(cdecl, rw_00b35580(first: u32, last: u32, pivot_bits: u32) -> u32 {
    unsafe {
        const ELEM: u32 = 16;
        const N_COPY: usize = 4;
        let pivot = f32::from_bits(pivot_bits);
        let mut lo = first;
        let mut hi = last;
        loop {
            loop {
                let v = f32::from_bits((lo as *const u32).read_unaligned());
                if !(pivot > v) {
                    break;
                }
                lo = lo.wrapping_add(ELEM);
            }
            loop {
                hi = hi.wrapping_sub(ELEM);
                let v = f32::from_bits((hi as *const u32).read_unaligned());
                if !(v > pivot) {
                    break;
                }
            }
            if lo >= hi {
                return lo;
            }
            let mut tmp = [0u32; N_COPY];
            core::ptr::copy_nonoverlapping(hi as *const u32, tmp.as_mut_ptr(), N_COPY);
            core::ptr::copy_nonoverlapping(
                lo as *const u32,
                hi as *mut u32,
                N_COPY,
            );
            core::ptr::copy_nonoverlapping(tmp.as_ptr(), lo as *mut u32, N_COPY);
            lo = lo.wrapping_add(ELEM);
        }
    }
});
