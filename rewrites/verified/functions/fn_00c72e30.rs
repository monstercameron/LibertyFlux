// original: 0x00C72E30 find_animation_record_or_null

/// Ask the record search helper for a signed index. Return null when that
/// index is negative; otherwise return the corresponding 0x110-byte record
/// address. The helper result is compared as signed, so every high-bit result
/// takes the null path.
lf_checker_rt::export!(cdecl, rw_00c72e30(key: u32) -> u32 {
    const SEARCH: u32 = 1;
    const RECORDS_BASE: u32 = 0x016d_d6a0;
    const RECORD_STRIDE: u32 = 0x110;
    let index = lf_checker_rt::callee_cdecl!(SEARCH, u32, key);
    if (index as i32) < 0 {
        0
    } else {
        lf_checker_rt::relocated(RECORDS_BASE).wrapping_add(index.wrapping_mul(RECORD_STRIDE))
    }
});
