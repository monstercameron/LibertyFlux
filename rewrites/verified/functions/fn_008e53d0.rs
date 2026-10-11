// original: 0x008E53D0 audio_event_map_upsert

/// Update an existing audio-event key or append a new key/value pair.
///
/// The method reads a signed pair count at `0x33C` and searches that many
/// eight-byte entries beginning at `0x23C`. An equal key updates the pair's
/// value. Otherwise it appends at the current count, increments the count,
/// and returns one, except that a count of exactly 32 returns zero. A
/// nonpositive count skips the search; the append address still follows the
/// original signed index arithmetic. This is a 32-bit thiscall method with
/// two stack words and the callee pops 8 bytes cleanup.
lf_checker_rt::export!(thiscall, rw_008e53d0(this: u32, key: u32, value: u32) -> u32 {
    unsafe {
        const COUNT_OFFSET: usize = 0x33c;
        const ENTRY_BASE: i32 = 0x23c;
        const ENTRY_STRIDE: i32 = 8;
        const ENTRY_LIMIT: i32 = 0x20;

        let count_ptr = (this as usize as *mut i32).add(COUNT_OFFSET / 4);
        let pair_count = count_ptr.read_unaligned();
        if pair_count > 0 {
            let mut index = 0i32;
            while index < pair_count {
                let entry_offset = ENTRY_BASE.wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
                let entry = (this as usize as *mut u8).offset(entry_offset as isize);
                if (entry as *const u32).read_unaligned() == key {
                    (entry.add(4) as *mut u32).write_unaligned(value);
                    return 1;
                }
                index += 1;
            }
        }
        if pair_count == ENTRY_LIMIT {
            return 0;
        }
        let entry_offset = ENTRY_BASE.wrapping_add(pair_count.wrapping_mul(ENTRY_STRIDE));
        let entry = (this as usize as *mut u8).offset(entry_offset as isize);
        count_ptr.write_unaligned(pair_count.wrapping_add(1));
        (entry as *mut u32).write_unaligned(key);
        (entry.add(4) as *mut u32).write_unaligned(value);
        1
    }
});
