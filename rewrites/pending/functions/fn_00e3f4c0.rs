// original: 0x00e3f4c0 SlotRun_LeadCount
// 0x00E3F4C0: count the leading run of free (-1) slots in a row table.
// (stdcall/2)
export!(stdcall, rw_00e3f4c0(base: u32, count: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x2b0;
        let n = count as i32;
        if n <= 0 {
            return 0;
        }
        let mut run = 0i32;
        let mut i = 0i32;
        while i < n {
            let tag = *((base
                .wrapping_add(0x14)
                .wrapping_add((i as u32).wrapping_mul(STRIDE)))
                as *const i32);
            if tag != -1 {
                break;
            }
            run += 1;
            i += 1;
        }
        if run > 0 {
            run as u32
        } else {
            0
        }
    }
});
