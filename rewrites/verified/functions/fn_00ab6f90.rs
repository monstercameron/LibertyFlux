// original: 0x00ab6f90 hash_checked_flag
/// Looks `key` up in the checked table (bucket = key % capacity, chain at
/// +0x14); on a hit resolves the entry through the loader and returns bit 0
/// of its tag. The second argument is ignored by the original.

export!(thiscall, rw_00ab6f90(this: *const u8, key: u32, fwd: u32) -> u32 {
    unsafe {
        let cap = *(this.add(0xC) as *const u16) as u32;
        if cap == 0 {
            return 0;
        }
        let buckets = *(this.add(8) as *const u32) as *const u32;
        let mut n = *buckets.add((key % cap) as usize);
        while n != 0 {
            if *(n as *const u32) == key {
                let obj = n.wrapping_add(4);
                if obj == 0 {
                    return 0;
                }
                let res: u32 = callee_thiscall!(1, u32, obj, fwd);
                if res == 0 {
                    return 0;
                }
                return ((*(res as *const u8)) & 1) as u32;
            }
            n = *((n + 0x14) as *const u32);
        }
        0
    }
});
