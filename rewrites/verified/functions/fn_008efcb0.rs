// original: 0x008efcb0 pool_entry_find
/// Find a row in one of the object's six pools: the kind selects the pool,
/// the search scans entries from the start index for the tag built from the
/// key plus the pool base, and a hit copies the row's payload words to the
/// two out pointers (masking the first to 24 bits) and returns the index.
/// Returns -1 when the kind is unknown, the pool is empty, or no tag matches.
export!(thiscall, rw_008efcb0(
    this: u32,
    kind: u32,
    key: u32,
    out0: u32,
    out1: u32,
    start: u32,
) -> u32 {
    const POOLS: [u32; 6] = [0, 0xF70, 0x7B8, 0x1EE0, 0x1728, 0x32AC];
    const TAG_BASE: u32 = 0x2698;
    const TAG_COL: u32 = 0x14A;
    if kind > 5 {
        return 0xFFFF_FFFF;
    }
    let pool = this.wrapping_add(POOLS[kind as usize]);
    if pool == 0 {
        return 0xFFFF_FFFF;
    }
    let count = unsafe { *(pool as *const i32) };
    let mut i = start as i32;
    if i >= count {
        return 0xFFFF_FFFF;
    }
    let tag = this
        .wrapping_add(TAG_BASE)
        .wrapping_add(key.wrapping_shl(4));
    loop {
        let entry = unsafe {
            *((pool.wrapping_add((i.wrapping_add(TAG_COL as i32) as u32).wrapping_mul(4)))
                as *const u32)
        };
        if entry == tag {
            if i == -1 {
                return 0xFFFF_FFFF;
            }
            let w0 = unsafe {
                *((pool.wrapping_add((i as u32).wrapping_mul(4)).wrapping_add(0x298))
                    as *const u32)
            };
            unsafe { *(out0 as *mut u32) = w0 };
            unsafe { *((out0.wrapping_add(3)) as *mut u8) = 0 };
            let w1 = unsafe {
                *((pool.wrapping_add((i as u32).wrapping_mul(4)).wrapping_add(8))
                    as *const u32)
            };
            unsafe { *(out1 as *mut u32) = w1 };
            return i as u32;
        }
        i = i.wrapping_add(1);
        if i >= count {
            return 0xFFFF_FFFF;
        }
    }
});
