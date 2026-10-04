// original: 0x00ab9930 slot_search_field4

/// Search two stride-64 records by key, return the dword at offset 4.
///
/// Returns the payload word of the first record whose header equals the key,
/// or all-ones when no record matches.
export!(thiscall, rs64_ab9930(this: *const u8, key: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 2;
        const STRIDE: u32 = 64;
        const MISS: u32 = 0xFFFF_FFFF;
        let base = this as u32;
        let mut i = 0u32;
        while i < COUNT {
            let slot = base.wrapping_add(i.wrapping_mul(STRIDE));
            if *(slot as *const u32) == key {
                return *((slot.wrapping_add(4)) as *const u32);
            }
            i += 1;
        }
        MISS
    }
});
