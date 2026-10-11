// original: 0x00C72E20 native_get_car_model_value_record

/// Return the record address for a signed model index using the original
/// 0x110-byte record stride. Address arithmetic wraps at 32 bits, including
/// negative or otherwise out-of-range indices; this helper does not read the
/// record or apply a bounds check.
lf_checker_rt::export!(cdecl, rw_00c72e20(index: u32) -> u32 {
    const RECORDS_BASE: u32 = 0x016d_d6a0;
    const RECORD_STRIDE: u32 = 0x110;
    lf_checker_rt::relocated(RECORDS_BASE).wrapping_add(index.wrapping_mul(RECORD_STRIDE))
});
