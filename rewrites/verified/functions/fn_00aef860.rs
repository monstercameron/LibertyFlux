// original: 0x00AEF860 stream_id_index_or_neg (proposed)

/// Index of the stream-table slot with a matching id, else -1.
///
/// Same 225-slot scan as the sibling index lookup, but a miss returns
/// -1, so slot 0 is unambiguous. See the sibling for the table layout.
///
/// Original: 0x00AEF860 (cdecl, one stack word, no calls).
lf_checker_rt::export!(cdecl, rw_00aef860(id: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x015DE3A8;
        const TABLE_END: u32 = 0x015E88E4;
        const STRIDE: u32 = 0xBC;
        const MISS: u32 = 0xFFFF_FFFF;
        let base = lf_checker_rt::relocated(TABLE);
        let end = lf_checker_rt::relocated(TABLE_END);
        let mut p = base;
        let mut i: u32 = 0;
        while p < end {
            if ((p) as *const u32).read_unaligned() == id {
                return i;
            }
            p = p.wrapping_add(STRIDE);
            i = i.wrapping_add(1);
        }
        MISS
    }
});
