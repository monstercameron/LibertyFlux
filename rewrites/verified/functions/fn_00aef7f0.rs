// original: 0x00AEF7F0 stream_id_index (proposed)

/// Index of the stream-table slot with a matching id, else 0.
///
/// Scans the 225 global slots (stride 0xBC) comparing each slot's first
/// word with `id`; returns the slot index of the first match, or 0 when
/// no slot matches (indistinguishable from a match at slot 0, exactly as
/// the original). The table lives in the image's data section and is
/// zero-filled on disk; the contract plants nonzero ids.
///
/// Original: 0x00AEF7F0 (cdecl, one stack word, no calls).
lf_checker_rt::export!(cdecl, rw_00aef7f0(id: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x015DE3A8;
        const TABLE_END: u32 = 0x015E88E4;
        const STRIDE: u32 = 0xBC;
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
        0
    }
});
