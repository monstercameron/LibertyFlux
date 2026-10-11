// original: 0x00c72e50 find_animation_record_index

/// Search the global animation-record table for `key`, comparing one 16-byte
/// record prefix at a time through the scripted comparison helper. The signed
/// global record count controls the loop: nonpositive counts return -1. A
/// zero helper result returns the matching zero-based index; otherwise the
/// function returns -1 after the table is exhausted.
lf_checker_rt::export!(cdecl, rw_00c72e50(key: u32) -> u32 {
    const GLOBAL_RECORD_COUNT: u32 = 0x016d_d690;
    const RECORDS_BASE: u32 = 0x016d_d6a0;
    const RECORD_STRIDE: u32 = 0x110;
    const COMPARE_PREFIX: u32 = 1;

    let count = unsafe { (lf_checker_rt::relocated(GLOBAL_RECORD_COUNT) as *const i32).read_unaligned() };
    if count <= 0 {
        return u32::MAX;
    }

    let records = lf_checker_rt::relocated(RECORDS_BASE);
    for index in 0..count as u32 {
        let record = records.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
        let same = lf_checker_rt::callee_cdecl!(COMPARE_PREFIX, u32, key, record, 0x10u32);
        if same == 0 {
            return index;
        }
    }
    u32::MAX
});
