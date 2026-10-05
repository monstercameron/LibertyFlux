// original: 0x00B3A280 table_range_check

/// Range test on a per-index record field: 1 when `field - 3 <= 11`
/// (unsigned), i.e. the field at `+0x90` lies in `3..=14`, else 0.
///
/// Cdecl, one stack word, returns 0/1 in eax.
///
/// Original: 0x00B3A280.

lf_checker_rt::export!(cdecl, rw_00B3A280(idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x01295CD8;
        const FIELD_OFF: u32 = 0x90;
        const LO: u32 = 3;
        const SPAN: u32 = 11;
        let rec = (lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let v = (rec.wrapping_add(FIELD_OFF) as *const u32).read_unaligned();
        u32::from(v.wrapping_sub(LO) <= SPAN)
    }
});
