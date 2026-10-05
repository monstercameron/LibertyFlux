// original: 0x00b510d0 find_slot_by_word1 (proposed)

/// Find the ped slot whose third word matches a key.
///
/// Scans the 120 slot records at `TABLE` (20 bytes each) comparing the word
/// at offset 8; returns the base address of the first match, or null when
/// no record matches.
///
/// Original: 0x00b510d0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00b510d0(key: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x016683a0;
        const TABLE_END: u32 = 0x01668da8;
        const STRIDE: u32 = 0x14;
        const KEY_OFF: u32 = 0x08;
        let mut p = TABLE + KEY_OFF;
        while p < TABLE_END {
            if ((lf_checker_rt::relocated(p)) as *const u32).read_unaligned() == key {
                return lf_checker_rt::relocated(p - KEY_OFF);
            }
            p = p.wrapping_add(STRIDE);
        }
        0
    }
});
