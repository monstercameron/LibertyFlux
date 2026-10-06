// original: 0x00693700 voice_table_lookup (proposed)

/// Looks up voice slot `idx` (signed) in the global voice table (table
/// pointer at file VA 0x19F2378, count as a 16-bit word at +4). Indices
/// below 5 (signed) index the table directly; otherwise entries from 5
/// below the count (signed) are scanned for one whose dword at +0 equals
/// the index, and null is returned when none matches or the count is 5 or
/// less. Null entries are skipped. Returns the entry pointer or null.
///
/// Original: 0x00693700 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00693700(idx: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x019F2378;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }

        let table = rd32(lf_checker_rt::relocated(TABLE));
        if (idx as i32) < 5 {
            return rd32(table.wrapping_add(idx.wrapping_mul(4)));
        }
        let count = rd16(lf_checker_rt::relocated(TABLE).wrapping_add(4)) as i32;
        if count <= 5 {
            return 0;
        }
        let mut i = 5i32;
        while i < count {
            let e = rd32(table.wrapping_add((i as u32).wrapping_mul(4)));
            if e != 0 && rd32(e) == idx {
                return e;
            }
            i += 1;
        }
        0
    }
});
