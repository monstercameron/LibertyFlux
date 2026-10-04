// original: 0x00e3f640 SlotTable_TailUsed
// 0x00E3F640: scan rows downward for the first used (not -1) slot,
// stopping early once the combined budget runs out. (stdcall/3)
export!(stdcall, rw_00e3f640(base: u32, count: u32, total: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        let t = total as i32;
        if t <= 0 {
            return 0;
        }
        let mut i = count as i32;
        if i > 0 {
            let steps = t.wrapping_sub(i);
            loop {
                if steps.wrapping_add(i) == 0 {
                    break;
                }
                let tag = *((base
                    .wrapping_add(0x14)
                    .wrapping_add((i as u32).wrapping_mul(STRIDE)))
                    as *const i32);
                if tag == -1 {
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
