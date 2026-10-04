// original: 0x0099d710 insertion_sort_records
/// Insertion pass over the 16-byte records in `[begin, end)`.
///
/// For each record after the first: if its key (dword at +12) is below the
/// current first record's key, the whole run shifts up one slot and the
/// record moves to the front; otherwise the record is inserted into the
/// sorted prefix by the sorted-insert helper (cdecl/6, stubbed). The third
/// word is padding the original never reads; the fourth is forwarded to the
/// helper with every call.
export!(cdecl, rw_0099d710(begin: u32, end: u32, _w3: u32, flag: u32) -> () {
    unsafe {
        if begin == end {
            return;
        }
        let mut cur = begin.wrapping_add(16);
        if cur == end {
            return;
        }
        while cur != end {
            let s = cur as *const u32;
            let r = [*s, *s.add(1), *s.add(2), *s.add(3)];
            let first_key = *((begin.wrapping_add(12)) as *const u32);
            if r[3] < first_key {
                let mut p = cur;
                while p != begin {
                    let d = p as *mut u32;
                    let q = p.wrapping_sub(16) as *const u32;
                    *d = *q;
                    *d.add(1) = *q.add(1);
                    *d.add(2) = *q.add(2);
                    *d.add(3) = *q.add(3);
                    p = p.wrapping_sub(16);
                }
                let d = begin as *mut u32;
                *d = r[0];
                *d.add(1) = r[1];
                *d.add(2) = r[2];
                *d.add(3) = r[3];
            } else {
                callee_cdecl!(1, u32, cur, r[0], r[1], r[2], r[3], flag);
            }
            cur = cur.wrapping_add(16);
        }
    }
});
