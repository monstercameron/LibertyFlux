// original: 0x00d6f150 replay_bar_scan_backward
/// Scan the slot table backward for the first entry scoring below `thresh`.
///
/// Writes 100 to `out_tag` and 0 to `out_stamp`, then scores each entry from
/// the end with callee 1 over its stamp (+0x14). On the first score strictly
/// below `thresh` the entry is scored again, its tag byte (+0x01) and stamp
/// go to the outputs, and the second score is returned. Returns 0 when the
/// table is empty or nothing scores below it.
lf_checker_rt::export!(thiscall, rw_00d6f150(
    this_ptr: u32,
    thresh: u32,
    out_tag: u32,
    out_stamp: u32,
) -> u32 {
    unsafe {
        *(out_tag as *mut u32) = 100;
        *(out_stamp as *mut u32) = 0;
        let b = this_ptr as *const u8;
        let table = *((b.add(0x9c)) as *const u32);
        let count = *(((table as *const u8).add(4)) as *const u16) as u32;
        let arr = *((table as *const u8) as *const u32);
        let end = (arr as u32).wrapping_add(count.wrapping_mul(4));
        if end == arr as u32 {
            return 0;
        }
        let mut p = end;
        loop {
            let elem = *(p.wrapping_sub(4) as *const u32);
            let stamp = *(((elem as *const u8).add(0x14)) as *const u32);
            let v: u32 = lf_checker_rt::callee_thiscall!(1, u32, this_ptr, stamp, 0);
            if v < thresh {
                let elem2 = *(p.wrapping_sub(4) as *const u32);
                let stamp2 = *(((elem2 as *const u8).add(0x14)) as *const u32);
                let v2: u32 =
                    lf_checker_rt::callee_thiscall!(1, u32, this_ptr, stamp2, 0);
                let tag = *(((elem2 as *const u8).add(1)) as *const u8);
                *(out_tag as *mut u32) = tag as u32;
                *(out_stamp as *mut u32) = stamp2;
                return v2;
            }
            p = p.wrapping_sub(4);
            if p == arr as u32 {
                return 0;
            }
        }
    }
});
