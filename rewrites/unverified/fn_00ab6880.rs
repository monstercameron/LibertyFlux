// original: 0x00ab6880 idmap_find_a (proposed)

/// Look up `key` in the chained id map and return the payload pointer.
///
/// The map header holds the bucket array at `+8` and the 16-bit bucket count
/// at `+0x0c`. A zero count means empty (returns null without dividing).
/// Otherwise bucket `key % count` is walked through the chain link at
/// `+0x14` until an entry whose word at `+0` equals the key; returns that
/// entry plus 4 (the payload), or null when the chain ends.
///
/// Original: 0x00ab6880 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ab6880(this: u32, key: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 8;
        const COUNT_OFF: u32 = 0x0C;
        const NEXT_OFF: u32 = 0x14;
        const PAYLOAD_OFF: u32 = 4;
        let count = ((this + COUNT_OFF) as *const u16).read_unaligned() as u32;
        if count == 0 {
            return 0;
        }
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        let mut entry = ((table + (key % count) * 4) as *const u32).read_unaligned();
        while entry != 0 {
            if (entry as *const u32).read_unaligned() == key {
                return entry.wrapping_add(PAYLOAD_OFF);
            }
            entry = ((entry + NEXT_OFF) as *const u32).read_unaligned();
        }
        0
    }
});
