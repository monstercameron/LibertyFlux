// original: 0x00932190 imposter_cache_find_free_slot

/// Return the first cache-record address whose key or value is the empty
/// sentinel. The fixed cache has sixteen 64-byte records. A record is free
/// when either of its first two words is `u32::MAX`; if none is free, return
/// null. The function takes no arguments and returns the relocated record
/// pointer in EAX.
lf_checker_rt::export!(cdecl, rw_00932190() -> u32 {
    const CACHE_BASE_VA: u32 = 0x011A_2A70;
    const RECORD_COUNT: u32 = 16;
    const RECORD_STRIDE: u32 = 64;
    const EMPTY_WORD: u32 = u32::MAX;

    let cache = lf_checker_rt::relocated(CACHE_BASE_VA);
    for record_index in 0..RECORD_COUNT {
        let record = cache.wrapping_add(record_index * RECORD_STRIDE);
        let key = unsafe { (record as *const u32).read_unaligned() };
        let value = unsafe { (record.wrapping_add(4) as *const u32).read_unaligned() };
        if key == EMPTY_WORD || value == EMPTY_WORD {
            return record;
        }
    }
    0
});
