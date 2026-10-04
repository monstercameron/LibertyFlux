// original: 0x00e3f5f0 SlotTable_FindKey
// 0x00E3F5F0: find the first used (not -1) row whose id matches the key.
// Returns the row index, or -1. (stdcall/3)
export!(stdcall, rw_00e3f5f0(base: u32, count: u32, key: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        let n = count as i32;
        if n <= 0 {
            return 0;
        }
        let mut i = 0i32;
        while i < n {
            let row = base.wrapping_add((i as u32).wrapping_mul(STRIDE));
            let tag = *((row.wrapping_add(0x14)) as *const i32);
            if tag != -1 && *((row.wrapping_add(0x10)) as *const u32) == key {
                return i as u32;
            }
            i += 1;
        }
        0xFFFF_FFFF
    }
});
