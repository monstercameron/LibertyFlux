// original: 0x00d6eff0 replay_bar_find_slot
/// Find the last slot whose stamp does not exceed the limit at +0xCC.
///
/// Scans the pointer table at +0x9C from the end and returns the highest
/// index whose entry stamp (+0x14) is at most the limit, or -1 (as u32)
/// when the table is empty or every stamp exceeds it.
lf_checker_rt::export!(thiscall, rw_00d6eff0(this_ptr: u32) -> u32 {
    unsafe {
        let b = this_ptr as *const u8;
        let table = *((b.add(0x9c)) as *const u32);
        let limit = *((b.add(0xcc)) as *const u32);
        let count = *(((table as *const u8).add(4)) as *const u16) as u32;
        let arr = *((table as *const u8) as *const u32);
        let mut i = count.wrapping_sub(1);
        let mut p = (arr as u32).wrapping_add(count.wrapping_mul(4));
        if p == arr as u32 {
            return i;
        }
        loop {
            p = p.wrapping_sub(4);
            let elem = *(p as *const u32);
            let stamp = *(((elem as *const u8).add(0x14)) as *const u32);
            if stamp <= limit {
                return i;
            }
            i = i.wrapping_sub(1);
            if p == arr as u32 {
                return i;
            }
        }
    }
});
