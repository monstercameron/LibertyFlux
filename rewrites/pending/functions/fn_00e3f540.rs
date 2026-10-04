// original: 0x00e3f540 SlotTable_LastUsed
// 0x00E3F540: find the highest used (not -1) slot below the count.
// (stdcall/2)
export!(stdcall, rw_00e3f540(base: u32, count: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        let n = count as i32;
        if n <= 0 {
            return 0;
        }
        let mut i = n.wrapping_sub(1);
        if i > 0 {
            loop {
                let tag = *((base
                    .wrapping_add(0x14)
                    .wrapping_add((i as u32).wrapping_mul(STRIDE)))
                    as *const i32);
                if tag != -1 {
                    break;
                }
                i -= 1;
                if i <= 0 {
                    break;
                }
            }
        }
        if i > 0 {
            i as u32
        } else {
            0
        }
    }
});
