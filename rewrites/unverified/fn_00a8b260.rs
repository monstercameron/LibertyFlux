// original: 0x00a8b260 pool_find_slot (proposed)

/// Scan the slot array while entries stay at or below the key.
///
/// `this` is the pool object (slot count at +0x9C4, entries of 0x64 bytes
/// from +0xBC) and `key` the search key (signed). Walks entries 1..=count
/// while each entry's first word does not exceed the key, tracking the
/// last index whose word differs from the previous entry's (starting
/// before the array). Returns that index when an entry exceeds the key,
/// or 0xFF when the scan completes (or the count is not positive). The
/// proof pins the count to 0..8.
///
/// Original: 0x00A8B260 (thiscall, one stack word; the true tail sits past
/// an early `ret`, reached by the scan's exit jump).
lf_checker_rt::export!(thiscall, rw_00a8b260(this: u32, key: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x9c4;
        const ENTRIES: u32 = 0xbc;
        const ENTRY_SIZE: u32 = 0x64;
        const FULL_SCAN: u32 = 0xff;
        let count = ((this + COUNT) as *const u32).read_unaligned();
        let limit = count.wrapping_add(1);
        let mut i = 1u32;
        let mut acc = 0u32;
        if limit > i {
            let mut entry = this.wrapping_add(ENTRIES);
            let want = key as i32;
            while i < limit {
                let first =
                    (entry as *const u32).read_unaligned();
                if want < first as i32 {
                    return acc;
                }
                let prev = (entry.wrapping_sub(ENTRY_SIZE) as *const u32)
                    .read_unaligned();
                if first != prev {
                    acc = i;
                }
                i = i.wrapping_add(1);
                entry = entry.wrapping_add(ENTRY_SIZE);
            }
        }
        FULL_SCAN
    }
});
