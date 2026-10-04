// original: 0x00e3f580 SlotTable_FindFree
// 0x00E3F580: scan rows upward for the first free (-1) slot, starting at
// an index and bounded by a limit index plus a step budget; the result
// clamps below the limit. (stdcall/4)
export!(stdcall, rw_00e3f580(base: u32, start: u32, limit: u32, budget: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        let mut left = budget as i32;
        if left <= 0 {
            return 0;
        }
        let lim = limit as i32;
        let top = lim.wrapping_sub(1);
        let mut i = start as i32;
        if i >= lim {
            return (if i < top { i } else { top }) as u32;
        }
        loop {
            let tag = *((base
                .wrapping_add(0x14)
                .wrapping_add((i as u32).wrapping_mul(STRIDE)))
                as *const i32);
            if tag == -1 {
                if i > 0 {
                    i -= 1;
                }
                return (if i < top { i } else { top }) as u32;
            }
            if left == 0 {
                return (if i < top { i } else { top }) as u32;
            }
            i += 1;
            left -= 1;
            if i >= lim {
                return (if i < top { i } else { top }) as u32;
            }
        }
    }
});
