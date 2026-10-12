// original: 0x00E63E00 reset_2072_marker_records
/// Clears the first dword and the low two bytes of the second word in each of 2,072 eight-byte marker records, preserving the remaining bytes. Returns the end address.
lf_checker_rt::export!(cdecl, rw_00E63E00() -> u32 {
    const TABLE_VA: u32 = 0x012142D0;
    const WRITE_COUNT: u32 = 2072;
    const RETURN_COUNT: u32 = 2072;
    const STRIDE: u32 = 8;
    let table = lf_checker_rt::relocated(TABLE_VA);
    for index in 0..WRITE_COUNT {
        let record = table.wrapping_add(index.wrapping_mul(STRIDE));
        unsafe {
            (record as *mut u32).write(0);
            (record.wrapping_add(4) as *mut u16).write(u16::MAX);
        }
    }
    table.wrapping_add(RETURN_COUNT.wrapping_mul(STRIDE))
});
