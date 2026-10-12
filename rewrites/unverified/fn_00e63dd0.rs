// original: 0x00E63DD0 clear_512_resource_records
/// Clears two dwords and one byte in each of 512 consecutive 12-byte resource records. The returned address is one stride beyond the final record.
lf_checker_rt::export!(cdecl, rw_00E63DD0() -> u32 {
    const TABLE_VA: u32 = 0x011FF078;
    const WRITE_COUNT: u32 = 512;
    const RETURN_COUNT: u32 = 512;
    const STRIDE: u32 = 12;
    let table = lf_checker_rt::relocated(TABLE_VA);
    for index in 0..WRITE_COUNT {
        let record = table.wrapping_add(index.wrapping_mul(STRIDE));
        unsafe {
            (record.wrapping_sub(8) as *mut u32).write(0);
            (record.wrapping_sub(4) as *mut u32).write(0);
            (record as *mut u8).write(0);
        }
    }
    table.wrapping_add(RETURN_COUNT.wrapping_mul(STRIDE))
});
