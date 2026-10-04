// original: 0x00ab7510 hash_table_lookup
/// Hash lookup of `key` (bucket = key % capacity, chain at +8); returns the
/// payload word after the node header, or the shared default address.

export!(thiscall, rw_00ab7510(this: *const u8, key: u32) -> u32 {
    unsafe {
        let cap = *(this.add(4) as *const u16) as u32;
        if cap == 0 {
            return relocated(0x150E12C);
        }
        let buckets = *(this as *const u32) as *const u32;
        let mut n = *buckets.add((key % cap) as usize);
        while n != 0 {
            if *(n as *const u32) == key {
                if n.wrapping_add(4) == 0 {
                    return relocated(0x150E12C);
                }
                return *((n + 4) as *const u32);
            }
            n = *((n + 8) as *const u32);
        }
        relocated(0x150E12C)
    }
});
