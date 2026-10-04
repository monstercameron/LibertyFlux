// original: 0x0094f8e0 find_or_alloc_table_entry
/// Find a table entry by key, allocating a free slot on a miss.
///
/// Scans the 4096-entry global table in order: the first slot whose key is
/// the free marker is filled with `key` and the info value and its index
/// returned; a slot already holding `key` is returned as-is. Returns
/// `0x1000` when no slot is free and no key matches.
export!(cdecl, rw_0094f8e0(key: u32, info: *const u8) -> u32 {
    unsafe {
        const TABLE: u32 = 0x012008B0;
        const ENTRIES: u32 = 0x1000;
        let mut i: u32 = 0;
        while i < ENTRIES {
            let slot = global::<u32>(TABLE + i * 8);
            let k = *slot;
            if k == 0xFFFF_FFFF {
                *slot = key;
                *slot.add(1) = *(info.add(0x68) as *const u32);
                return i;
            }
            if k == key {
                return i;
            }
            i += 1;
        }
        i
    }
});
