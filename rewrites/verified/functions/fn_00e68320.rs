// original: 0x00e68320 veh_table_fill_neg1_8320

/// Fill a vehicle table with -1 words (`rep stosd` of 0xC4 dwords).
///
/// Writes `FILL_WORD` to `TABLE_WORDS` consecutive dwords starting at
/// `TABLE_BASE` in static storage. Takes no arguments, makes no calls and
/// returns nothing.
///
/// Original: 0x00E68320 (cdecl/0, global writes only).
lf_checker_rt::export!(cdecl, rw_00e68320() -> () {
    unsafe {
        /// First word of the table (file VA).
        const TABLE_BASE: u32 = 0x012FAE88;
        /// Words in the table.
        const TABLE_WORDS: u32 = 0xC4;
        /// Fill value.
        const FILL_WORD: u32 = 0xFFFF_FFFF;
        let base = lf_checker_rt::relocated(TABLE_BASE);
        let mut i = 0u32;
        while i < TABLE_WORDS {
            core::ptr::write_unaligned((base.wrapping_add(i.wrapping_mul(4))) as *mut u32, FILL_WORD);
            i += 1;
        }
    }
});
