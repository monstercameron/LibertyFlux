// original: 0x00e682e0 veh_table_zero_82e0

/// Zero a 60-word vehicle table in static storage.
///
/// The original walks twelve groups of five dwords (`TABLE_BASE` plus
/// `GROUP_STRIDE` per group, five stores per group) writing zero; the effect
/// is 240 zeroed bytes at `TABLE_BASE`. Takes no arguments, makes no calls
/// and returns nothing.
///
/// Original: 0x00E682E0 (cdecl/0, global writes only).
lf_checker_rt::export!(cdecl, rw_00e682e0() -> () {
    unsafe {
        /// First word of the table (file VA).
        const TABLE_BASE: u32 = 0x012FA758;
        /// Words in the table.
        const TABLE_WORDS: u32 = 60;
        let base = lf_checker_rt::relocated(TABLE_BASE);
        let mut i = 0u32;
        while i < TABLE_WORDS {
            core::ptr::write_unaligned((base.wrapping_add(i.wrapping_mul(4))) as *mut u32, 0);
            i += 1;
        }
    }
});
