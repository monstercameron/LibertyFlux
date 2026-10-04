// original: 0x0094eec0 clear_counter_pairs_16
/// Zero 16 pairs of counters in two adjacent arrays.
///
/// Clears words `0..16` and words `16..32` (bytes `0x40..0x80`) and returns
/// the count 16. (The batch lists 26 bytes for this function, but the loop
/// runs to offset 38; the rewrite follows the code, not the listing.)
export!(thiscall, rw_0094eec0(counts: *mut u32) -> u32 {
    unsafe {
        for i in 0..0x10 {
            *counts.add(i) = 0;
            *counts.add(i + 0x10) = 0;
        }
        0x10
    }
});
