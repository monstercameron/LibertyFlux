// original: 0x00B3A2A0 table_flag_is_set

/// Test bit 1 of a per-index record: 1 when the bit is set, 0 when clear.
///
/// Same table and flag word as `rw_00B3A250`, opposite sense. Cdecl, one
/// stack word, returns 0/1 in eax.
///
/// Original: 0x00B3A2A0.

lf_checker_rt::export!(cdecl, rw_00B3A2A0(idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x01295CD8;
        const FLAG_OFF: u32 = 0x120;
        const FLAG_BIT: u32 = 2;
        let rec = (lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let flags = (rec.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
        u32::from(flags & FLAG_BIT != 0)
    }
});
