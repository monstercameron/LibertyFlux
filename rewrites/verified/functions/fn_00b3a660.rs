// original: 0x00B3A660 word_table_contains

/// Search the word list of index `idx` for `target`.
///
/// The count table at `G_COUNTS` gives the list length (signed); list `idx`
/// holds 16-bit entries at `G_LISTS + idx * 80`. Every exit writes only
/// `al`, so the upper 24 bits of the answer are behaviour: the index word
/// on the empty-list path, the scan pointer (masked) on a miss, and the
/// match pointer (masked) with bit 0 set on a hit. Cdecl, two stack words.
///
/// Original: 0x00B3A660.

lf_checker_rt::export!(cdecl, rw_00B3A660(idx: u32, target: u32) -> u32 {
    unsafe {
        const G_COUNTS: u32 = 0x0169E248;
        const G_LISTS: u32 = 0x0169C488;
        const STRIDE: u32 = 80;
        let count = (lf_checker_rt::relocated(G_COUNTS).wrapping_add(idx.wrapping_mul(4))
            as *const i32)
            .read_unaligned();
        if count <= 0 {
            return idx & 0xFFFF_FF00;
        }
        let base = lf_checker_rt::relocated(G_LISTS).wrapping_add(idx.wrapping_mul(STRIDE));
        let mut i = 0i32;
        while i < count {
            let at = base.wrapping_add((i as u32).wrapping_mul(2));
            let w = (at as *const u16).read_unaligned();
            if w as u32 == target {
                return (at & 0xFFFF_FF00) | 1;
            }
            i += 1;
        }
        (base.wrapping_add((count as u32).wrapping_mul(2)) & 0xFFFF_FF00)
    }
});
