// original: 0x00CC7240 euphoria_teardown_lists (proposed)

/// Free both global record lists and the global pointer array, then clear them.
///
/// Frees each record of the two global lists (skipping null records), drains
/// both queues through the drain callee, then walks the global pointer array:
/// each non-null entry whose half-word at `+6` is non-zero has its first word
/// freed before the entry itself is freed. Finally the array base is freed
/// and the array base and count globals are zeroed. Returns zero.
///
/// Original: 0x00CC7240 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00cc7240() -> u32 {
    unsafe {
        const LIST_A: u32 = 0x0171C0F8;
        const LIST_B: u32 = 0x0171C110;
        const ARRAY_BASE: u32 = 0x0171C104;
        const ARRAY_COUNT: u32 = 0x0171C108;
        const NEXT: u32 = 4;
        const MARK_AT: u32 = 6;
        const FREE_CALLEE: u32 = 1;
        const DRAIN_CALLEE: u32 = 2;
        #[inline(always)]
        unsafe fn rd(addr: u32) -> u32 {
            unsafe { (addr as *const u32).read_unaligned() }
        }
        for list in [LIST_A, LIST_B] {
            let head = lf_checker_rt::relocated(list);
            let mut node = rd(head);
            while node != 0 {
                let record = rd(node);
                if record != 0 {
                    lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, record);
                }
                node = rd(node.wrapping_add(NEXT));
            }
            lf_checker_rt::callee_thiscall!(DRAIN_CALLEE, u32, head);
        }
        let base_addr = lf_checker_rt::relocated(ARRAY_BASE);
        let count_addr = lf_checker_rt::relocated(ARRAY_COUNT);
        let count = (count_addr as *const u16).read_unaligned() as u32;
        // The original tests 0 against the count unsigned, then loops signed.
        if count != 0 {
            let mut i = 0u32;
            while (i as i32) < (count as i32) {
                let entry =
                    rd(rd(base_addr).wrapping_add(i.wrapping_mul(4)));
                if entry != 0 {
                    if (entry.wrapping_add(MARK_AT) as *const u16).read_unaligned() != 0 {
                        lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, rd(entry));
                    }
                    lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, entry);
                }
                i = i.wrapping_add(1);
            }
        }
        lf_checker_rt::callee_cdecl!(FREE_CALLEE, u32, rd(base_addr));
        (base_addr as *mut u32).write_unaligned(0);
        (count_addr as *mut u32).write_unaligned(0);
        0
    }
});
