// original: 0x00b51150 find_empty_slot (proposed)

/// Find the first ped slot whose leading three words are all zero.
///
/// Scans the 120 slot records at `TABLE` (20 bytes each); returns the index
/// of the first record with zero at offsets 4, 8 and 12, or -1 when every
/// record is in use.
///
/// Original: 0x00b51150 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00b51150() -> u32 {
    unsafe {
        const TABLE: u32 = 0x016683a0;
        const TABLE_END: u32 = 0x01668da8;
        const STRIDE: u32 = 0x14;
        let mut p = TABLE + 0x08;
        let mut i: u32 = 0;
        while p < TABLE_END {
            let b = lf_checker_rt::relocated(p);
            if ((b - 4) as *const u32).read_unaligned() == 0
                && (b as *const u32).read_unaligned() == 0
                && ((b + 4) as *const u32).read_unaligned() == 0
            {
                return i;
            }
            p = p.wrapping_add(STRIDE);
            i = i.wrapping_add(1);
        }
        0xffff_ffff
    }
});
