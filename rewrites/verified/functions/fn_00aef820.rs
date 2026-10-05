// original: 0x00AEF820 stream_id_factor (proposed)

/// Float stored at offset 0x48 of the matching stream-table slot, else 1.0.
///
/// Scans the 225 slots for `id` like the sibling lookups; on a hit the
/// single at slot offset 0x48 is returned (on the x87 stack), on a miss
/// the answer is 1.0.
///
/// Original: 0x00AEF820 (cdecl, one stack word, no calls).
lf_checker_rt::export!(cdecl, rw_00aef820(id: u32) -> f32 {
    unsafe {
        const TABLE: u32 = 0x015DE3A8;
        const TABLE_END: u32 = 0x015E88E4;
        const STRIDE: u32 = 0xBC;
        const FACTOR_OFF: u32 = 0x48;
        let base = lf_checker_rt::relocated(TABLE);
        let end = lf_checker_rt::relocated(TABLE_END);
        let mut p = base;
        while p < end {
            if ((p) as *const u32).read_unaligned() == id {
                return f32::from_bits(((p.wrapping_add(FACTOR_OFF)) as *const u32).read_unaligned());
            }
            p = p.wrapping_add(STRIDE);
        }
        1.0
    }
});
