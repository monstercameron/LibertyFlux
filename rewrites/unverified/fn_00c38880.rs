// original: 0x00C38880 zone_table_row_for_mode

/// Return the row pointer for one of three zone tables. Modes zero, one, and
/// two select their corresponding table; mode three and any larger unsigned
/// mode return zero. Each row occupies twelve bytes, and the second cdecl
/// argument is used as a wrapping 32-bit row index.
lf_checker_rt::export!(cdecl, rw_00c38880(mode: u32, row_index: u32) -> u32 {
    const MODE_ZERO_TABLE_VA: u32 = 0x01048508;
    const MODE_ONE_TABLE_VA: u32 = 0x010485c8;
    const MODE_TWO_TABLE_VA: u32 = 0x01048628;
    const ROW_SIZE: u32 = 12;

    let table_file_va = match mode {
        0 => MODE_ZERO_TABLE_VA,
        1 => MODE_ONE_TABLE_VA,
        2 => MODE_TWO_TABLE_VA,
        _ => return 0,
    };
    lf_checker_rt::relocated(table_file_va).wrapping_add(row_index.wrapping_mul(ROW_SIZE))
});
