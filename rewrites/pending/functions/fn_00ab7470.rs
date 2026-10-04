// original: 0x00ab7470 hash_resolve_field28
/// Looks `key` up in the object table (chain at +8); on a hit resolves the
/// entry's object and returns the dword at +0x1C of the resolution. The third
/// argument is ignored by the original.

export!(thiscall, rw_00ab7470(this: *const u8, key: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let cap = *(this.add(4) as *const u16) as u32;
        if cap == 0 {
            return 0;
        }
        let buckets = *(this as *const u32) as *const u32;
        let mut n = *buckets.add((key % cap) as usize);
        while n != 0 {
            if *(n as *const u32) == key {
                if n.wrapping_add(4) == 0 {
                    return 0;
                }
                let obj = *((n + 4) as *const u32);
                if obj == 0 {
                    return 0;
                }
                let res: u32 = callee_thiscall!(1, u32, obj, a1, a2);
                return *((res + 0x1C) as *const u32);
            }
            n = *((n + 8) as *const u32);
        }
        0
    }
});
