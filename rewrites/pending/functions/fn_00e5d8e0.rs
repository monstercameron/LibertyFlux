// original: 0x00e5d8e0 clear_32_entry_table
/// Zero the 32-entry table of 12-byte records and return the end pointer.
///
/// Each record holds three zeroed dwords; the returned pointer is one
/// record past the last one, matching the counter the original leaves behind.
lf_checker_rt::export!(cdecl, rw_00e5d8e0() -> u32 {
    unsafe {
        let mut cursor = lf_checker_rt::global::<u32>(0x019d2a48);
        for _ in 0..32u32 {
            *cursor.sub(2) = 0;
            *cursor.sub(1) = 0;
            *cursor = 0;
            cursor = cursor.add(3);
        }
        cursor as u32
    }
});
