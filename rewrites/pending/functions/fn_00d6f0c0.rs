// original: 0x00d6f0c0 replay_bar_scan_forward
/// Scan the slot table forward for the first entry scoring above `thresh`.
///
/// Writes 100 to `out`, then scores each entry with callee 1 over its stamp
/// (+0x14). On the first score strictly above `thresh` the entry is scored
/// again, its tag byte (+0x01) is stored to `out`, and the second score is
/// returned. Returns the fallback at +0x100 when nothing scores above it.
lf_checker_rt::export!(thiscall, rw_00d6f0c0(this_ptr: u32, thresh: u32, out: u32) -> u32 {
    unsafe {
        *(out as *mut u32) = 100;
        let b = this_ptr as *const u8;
        let table = *((b.add(0x9c)) as *const u32);
        let fallback = *((b.add(0x100)) as *const u32);
        let count = *(((table as *const u8).add(4)) as *const u16) as u32;
        let arr = *((table as *const u8) as *const u32);
        let end = (arr as u32).wrapping_add(count.wrapping_mul(4));
        if arr as u32 == end {
            return fallback;
        }
        let mut p = arr as u32;
        loop {
            let elem = *(p as *const u32);
            let stamp = *(((elem as *const u8).add(0x14)) as *const u32);
            let v: u32 = lf_checker_rt::callee_thiscall!(1, u32, this_ptr, stamp, 0);
            if v > thresh {
                let elem2 = *(p as *const u32);
                let stamp2 = *(((elem2 as *const u8).add(0x14)) as *const u32);
                let v2: u32 =
                    lf_checker_rt::callee_thiscall!(1, u32, this_ptr, stamp2, 0);
                let tag = *(((elem2 as *const u8).add(1)) as *const u8);
                *(out as *mut u32) = tag as u32;
                return v2;
            }
            p = p.wrapping_add(4);
            if p == end {
                return fallback;
            }
        }
    }
});
