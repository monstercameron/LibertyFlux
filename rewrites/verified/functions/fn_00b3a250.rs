// original: 0x00B3A250 table_flag_is_clear

/// Test bit 1 of a per-index record: 1 when the bit is clear, 0 when set.
///
/// `TABLE[idx]` points at a record; the flag word is at `+0x120`. Cdecl, one
/// stack word, returns 0/1 in eax.
///
/// Original: 0x00B3A250.

lf_checker_rt::export!(cdecl, rw_00B3A250(idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x01295CD8;
        const FLAG_OFF: u32 = 0x120;
        const FLAG_BIT: u32 = 2;
        let rec = (lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(4)) as *const u32)
            .read_unaligned();
        let flags = (rec.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
        u32::from(flags & FLAG_BIT == 0)
    }
});
