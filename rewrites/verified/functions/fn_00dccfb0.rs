// original: 0x00DCCFB0 STAT_UNKNOWN
/// Looks up a signed statistic identifier in one of four absolute data
/// tables and returns the table value when nonzero. Negative identifiers
/// and identifiers at or above 685 return the relocated unknown-name
/// string. The table ranges are [0,253), [253,649), [649,682), [682,685).
lf_checker_rt::export!(cdecl, rw_00dccfb0(stat_id: i32) -> u32 {
    const TABLE_A_VA: u32 = 0x00ef55f8;
    const TABLE_B_VA: u32 = 0x00ef6934;
    const TABLE_C_VA: u32 = 0x00ef8894;
    const TABLE_D_VA: u32 = 0x00ef8b70;
    const UNKNOWN_NAME_VA: u32 = 0x00ef9654;
    const RANGE_A_END: i32 = 253;
    const RANGE_B_END: i32 = 649;
    const RANGE_C_END: i32 = 682;
    const RANGE_D_END: i32 = 685;

    let value = stat_id;
    let unknown = lf_checker_rt::global::<u8>(UNKNOWN_NAME_VA) as u32;
    if value < 0 || value >= RANGE_D_END {
        return unknown;
    }
    let (table_va, index) = if value < RANGE_A_END {
        (TABLE_A_VA, value as u32)
    } else if value < RANGE_B_END {
        (TABLE_B_VA, value as u32)
    } else if value < RANGE_C_END {
        (TABLE_C_VA, value as u32)
    } else {
        (TABLE_D_VA, value as u32)
    };
    // SAFETY: the selected bounded identifier addresses one relocated word
    // in the corresponding read-only data table.
    let label = unsafe {
        lf_checker_rt::global::<u32>(table_va.wrapping_add(index.wrapping_mul(4))).read()
    };
    if label == 0 { unknown } else { label }
});
