// original: 0x00883260 stream_find_codec_entry (proposed)
/// Look up a codec entry by name: resolve the name, then scan the codec table.
///
/// Resolves `name` through the name lookup (intercepted callee 1, cdecl:
/// `name`, then tag `0x2e`); a null answer fails with null. Otherwise builds
/// a four-byte key from bytes 0, 2, 4 and 6 of the answer (plus a zero fifth
/// byte) and scans the six codec records starting at file address
/// `0x1030040` (12 bytes apart), comparing the key against each record
/// through the key compare routine (intercepted callee 2, cdecl: record
/// address, key address). The first record that compares equal (answer zero)
/// selects row `3 * position` from the value table at file address
/// `0x1030048`, returned to the caller; no match returns null.
///
/// Original: cdecl, one stack argument, returns the value (or null) in `eax`.
lf_checker_rt::export!(cdecl, rw_00883260(name: u32) -> u32 {
    unsafe {
        const NAME_TAG: u32 = 0x2e;
        const RECORD_BASE: u32 = 0x0103_0040;
        const VALUE_BASE: u32 = 0x0103_0048;
        const RECORD_STRIDE: u32 = 12;
        const RECORD_COUNT: u32 = 6;
        const ROW_STRIDE: u32 = 3;
        const LOOKUP_CALLEE: u32 = 1;
        const COMPARE_CALLEE: u32 = 2;
        let resolved: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, name, NAME_TAG);
        if resolved == 0 {
            return 0;
        }
        // Eight zeroed bytes: the contract snapshots two words at the key.
        let mut key = [0u8; 8];
        key[0] = (resolved as *const u8).read();
        key[1] = ((resolved + 2) as *const u8).read();
        key[2] = ((resolved + 4) as *const u8).read();
        key[3] = ((resolved + 6) as *const u8).read();
        let mut pos = 0u32;
        while pos < RECORD_COUNT {
            let record = lf_checker_rt::relocated(RECORD_BASE).wrapping_add(pos.wrapping_mul(RECORD_STRIDE));
            let same: u32 = lf_checker_rt::callee_cdecl!(
                COMPARE_CALLEE,
                u32,
                record,
                key.as_ptr() as u32
            );
            if same == 0 {
                let row = pos.wrapping_mul(ROW_STRIDE);
                let values = lf_checker_rt::relocated(VALUE_BASE);
                return ((values + row.wrapping_mul(4)) as *const u32).read_unaligned();
            }
            pos += 1;
        }
        0
    }
});
