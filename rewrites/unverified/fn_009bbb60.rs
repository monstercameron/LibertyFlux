// original: 0x009bbb60 find_free_input_slot (proposed)

/// Find the first free input slot in the 26-entry table.
///
/// Scans the table at 0x0128E960 (26 entries of 0x40 bytes) for the first
/// entry whose flag byte (`+0x00`) is zero and returns its index. When every
/// flag is non-zero returns -1. Reads only; writes nothing.
///
/// Edge cases: a full table returns -1; only the flag bytes steer the scan.
///
/// Original: no arguments, returns the index (or -1) in EAX.
lf_checker_rt::export!(cdecl, rw_009bbb60() -> u32 {
    unsafe {
        const TABLE: u32 = 0x0128e960;
        const END: u32 = 0x0128f5e0;
        const STRIDE: u32 = 0x40;
        let end = lf_checker_rt::relocated(END);
        let mut cur = lf_checker_rt::relocated(TABLE);
        let mut idx: u32 = 0;
        while cur < end {
            if (cur as *const u8).read() == 0 {
                return idx;
            }
            idx += 1;
            cur += STRIDE;
        }
        0xFFFFFFFF
    }
});
