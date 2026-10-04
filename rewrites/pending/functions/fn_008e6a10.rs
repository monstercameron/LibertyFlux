// original: 0x008e6a10 reinsert_each_entry
/// Call the single-element insert helper (cdecl/3, stubbed) for every entry
/// in `[begin, end)`. Returns the last helper answer.
export!(cdecl, rw_008e6a10(begin: *mut u8, end: *const u8, _extra: u32) -> u32 {
    unsafe {
        let mut cur = begin;
        let mut answer = 0u32;
        while cur as *const u8 != end {
            let e = entry_at(cur, 0);
            answer = callee_cdecl!(1, u32, cur as u32, e.0, e.1);
            cur = cur.add(8);
        }
        answer
    }
});

