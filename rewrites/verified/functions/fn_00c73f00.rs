// original: 0x00c73f00 register_animation_stream_block

/// Look up an animation record for `query`. The search helper's exact -1
/// sentinel returns unchanged. For any other index, require that the record's
/// pool field still names the expected pool and that the signed pool count is
/// below 40; then store the next 0xe0-byte pool slot in the record, increment
/// the global count, and return the index. Failed validation returns -1.
lf_checker_rt::export!(cdecl, rw_00c73f00(query: u32) -> u32 {
    const SEARCH_RECORD: u32 = 1;
    const GLOBAL_POOL_COUNT: u32 = 0x016d_d69c;
    const RECORD_POOL_FIELD: u32 = 0x016d_d7a0;
    const EXPECTED_POOL: u32 = 0x016e_99a0;
    const RECORD_STRIDE: u32 = 0x110;
    const POOL_STRIDE: u32 = 0xe0;
    const POOL_LIMIT: i32 = 0x28;

    let index = lf_checker_rt::callee_cdecl!(SEARCH_RECORD, u32, query);
    if index == u32::MAX {
        return u32::MAX;
    }

    unsafe {
        let record_field = lf_checker_rt::relocated(RECORD_POOL_FIELD)
            .wrapping_add(index.wrapping_mul(RECORD_STRIDE));
        if (record_field as *const u32).read_unaligned() != lf_checker_rt::relocated(EXPECTED_POOL) {
            return u32::MAX;
        }

        let count_address = lf_checker_rt::relocated(GLOBAL_POOL_COUNT);
        let count = (count_address as *const i32).read_unaligned();
        if count >= POOL_LIMIT {
            return u32::MAX;
        }

        let next_pool_slot = lf_checker_rt::relocated(EXPECTED_POOL)
            .wrapping_add((count as u32).wrapping_mul(POOL_STRIDE));
        (record_field as *mut u32).write_unaligned(next_pool_slot);
        (count_address as *mut i32).write_unaligned(count.wrapping_add(1));
    }
    index
});
