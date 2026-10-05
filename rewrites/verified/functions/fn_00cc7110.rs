// original: 0x00CC7110 euphoria_notify_tagged (proposed)

/// Notify every enumerated object whose tag appears in the global watch list.
///
/// Walks every object the enumerator callees yield. For each one, the global
/// -1-terminated watch list is scanned from the start, and the notify callee
/// runs once per watch entry equal to the object's tag (`+0xc`). An empty
/// watch list (first entry -1) skips the scan. Returns nothing meaningful.
///
/// Original: 0x00CC7110 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00cc7110(obj: u32) -> u32 {
    unsafe {
        const FIRST_CALLEE: u32 = 1;
        const NOTIFY_CALLEE: u32 = 2;
        const NEXT_CALLEE: u32 = 3;
        const TAG_AT: u32 = 0x0c;
        const WATCH_LIST: u32 = 0x01051598;
        const END_MARK: u32 = 0xFFFF_FFFF;
        let watch = lf_checker_rt::relocated(WATCH_LIST);
        let mut cand = lf_checker_rt::callee_thiscall!(FIRST_CALLEE, u32, obj, 0, 2);
        if cand == 0 {
            return 0;
        }
        loop {
            if (watch as *const u32).read_unaligned() != END_MARK {
                let mut i = 0u32;
                loop {
                    let entry =
                        (watch.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned();
                    if (cand.wrapping_add(TAG_AT) as *const u32).read_unaligned() == entry {
                        lf_checker_rt::callee_thiscall!(NOTIFY_CALLEE, u32, obj, cand);
                    }
                    i = i.wrapping_add(1);
                    if (watch.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned()
                        == END_MARK
                    {
                        break;
                    }
                }
            }
            cand = lf_checker_rt::callee_thiscall!(NEXT_CALLEE, u32, obj, 0, 2);
            if cand == 0 {
                return 0;
            }
        }
    }
});
