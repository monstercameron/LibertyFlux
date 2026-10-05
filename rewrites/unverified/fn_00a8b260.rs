// original: 0x00a8b260 pool_find_slot_always_ff (proposed)

/// Scan the slot array for the last entry not below the key; return 0xFF.
///
/// `this` is the pool object (slot count at +0x9C4, entries of 0x64 bytes
/// from +0xBC) and `key` the search key. Walks every live entry comparing
/// its first word against the key, then returns the constant 0xFF: the
/// accumulated match is dead, but the reads are kept so fault behaviour
/// matches the original. The proof pins the count to 0..8.
///
/// Original: 0x00A8B260 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8b260(this: u32, key: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x9c4;
        const ENTRIES: u32 = 0xbc;
        const ENTRY_SIZE: u32 = 0x64;
        const RESULT: u32 = 0xff;
        let count = ((this + COUNT) as *const u32).read_unaligned();
        let limit = count.wrapping_add(1);
        let mut i = 1u32;
        if limit > i {
            let mut entry = this.wrapping_add(ENTRIES);
            let _key = key as i32;
            while i < limit {
                let first = (entry as *const u32).read_unaligned();
                if _key < first as i32 {
                    // Below the key: no match, like the original's skip.
                } else {
                    let prev =
                        (entry.wrapping_sub(ENTRY_SIZE) as *const u32)
                            .read_unaligned();
                    core::hint::black_box(first ^ prev);
                }
                i = i.wrapping_add(1);
                entry = entry.wrapping_add(ENTRY_SIZE);
            }
        }
        RESULT
    }
});
