// original: 0x00CC5590 euphoria_find_feedback_by_key (proposed)

/// Find the feedback record whose head word equals `key`, clearing its notify flag.
///
/// Walks the global record list (nodes hold a record pointer and a next link).
/// Each record's first word is compared against `key`; the first match wins.
/// On a match, when the record's flag word at `+0x378` has bit 12 set, the
/// notify callee runs on the record and the bit is cleared (the record pointer
/// is re-read after the call, as the original does). Returns the matched
/// record, or null when the list is empty or nothing matches.
///
/// Original: 0x00CC5590 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00cc5590(key: u32) -> u32 {
    unsafe {
        const LIST_HEAD: u32 = 0x0171C0F8;
        const NEXT: u32 = 4;
        const FLAG_WORD: u32 = 0x378;
        const NOTIFY_BIT: u32 = 0x1000;
        const NOTIFY_CALLEE: u32 = 1;
        #[inline(always)]
        unsafe fn rd(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        let mut node = rd(lf_checker_rt::relocated(LIST_HEAD));
        while node != 0 {
            let record = rd(node);
            if rd(record) == key {
                if rd(record.wrapping_add(FLAG_WORD)) & NOTIFY_BIT != 0 {
                    lf_checker_rt::callee_cdecl!(NOTIFY_CALLEE, u32, record);
                    let fresh = rd(node);
                    let flags = rd(fresh.wrapping_add(FLAG_WORD));
                    (fresh.wrapping_add(FLAG_WORD) as *mut u32)
                        .write_unaligned(flags & !NOTIFY_BIT);
                }
                return rd(node);
            }
            node = rd(node.wrapping_add(NEXT));
        }
        0
    }
});
