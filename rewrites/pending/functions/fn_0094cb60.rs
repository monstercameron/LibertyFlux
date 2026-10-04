// original: 0x0094cb60 init_record_pool_100
/// Initialise a pool of 100 fixed-size records.
///
/// Each 48-byte record gets a tag of 1 in its first half-word and zeros
/// everywhere else (bytes 2-3 of each record are left untouched). Returns the
/// cursor one past the pool (base + 8 + 100 * stride), matching the
/// original's exit EAX.
export!(thiscall, rw_0094cb60(pool: *mut u8) -> u32 {
    unsafe {
        const ENTRIES: usize = 100;
        const STRIDE: usize = 0x30;
        for i in 0..ENTRIES {
            let e = pool.add(i * STRIDE);
            *(e as *mut u16) = 1;
            for w in 1..12 {
                *(e.add(w * 4) as *mut u32) = 0;
            }
        }
        // The original seeds its cursor 8 bytes past the base before the
        // loop, so it exits 8 bytes past the pool end, not exactly at it.
        pool as u32 + 8 + (ENTRIES * STRIDE) as u32
    }
});
