// original: 0x00C1FBE0 streaming_append_pending_request

/// Append one four-word request record while the global request count is not
/// four. The record stores the three supplied float bit patterns followed by
/// the integer identifier, at a sixteen-byte stride. On success the count is
/// incremented and one is returned in AL; at the exact capacity of four, no
/// global is written and zero is returned. The four arguments use cdecl word
/// slots, with the float values copied as raw bits.
lf_checker_rt::export!(cdecl, rw_00c1fbe0(identifier: u32, first_value: u32, second_value: u32, third_value: u32) -> u32 {
    unsafe {
        const COUNT_VA: u32 = 0x010482CC;
        const REQUESTS_VA: u32 = 0x016B9D78;
        const CAPACITY: u32 = 4;
        const RECORD_STRIDE: u32 = 16;
        let count_address = lf_checker_rt::relocated(COUNT_VA);
        let count = (count_address as *const u32).read_unaligned();
        if count == CAPACITY {
            return 0;
        }

        let record = lf_checker_rt::relocated(REQUESTS_VA)
            .wrapping_add(count.wrapping_mul(RECORD_STRIDE));
        (record as *mut u32).write_unaligned(first_value);
        (record.wrapping_add(4) as *mut u32).write_unaligned(second_value);
        (record.wrapping_add(8) as *mut u32).write_unaligned(third_value);
        (record.wrapping_add(12) as *mut u32).write_unaligned(identifier);
        (count_address as *mut u32).write_unaligned(count.wrapping_add(1));
        1
    }
});
