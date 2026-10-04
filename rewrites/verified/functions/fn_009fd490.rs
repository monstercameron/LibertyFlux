// original: 0x009FD490 frag_triple_table_find (proposed)

/// Search the global frag triple table for a row matching `key` and state.
///
/// Rows are 12 bytes: a key dword at offset 0 and a value dword at offset 8
/// of each row, with the row count in a global. For each row whose key
/// equals `key`, the value is compared against the dword at `+0x4c` of the
/// object a global points to; returns 1 on the first row where both match,
/// else 0. The state object is only dereferenced when a key matches, and
/// nothing is read when the count is zero.
///
/// Original: 0x009FD490 (cdecl, one stack word; boolean in `al`).
lf_checker_rt::export!(cdecl, rw_009FD490(key: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x012BCC90;
        const TABLE: u32 = 0x012B9C90;
        const STATE_PTR: u32 = 0x018B8968;
        const STATE_OFF: u32 = 0x4C;
        const STRIDE: u32 = 12;
        let count = (lf_checker_rt::global::<u32>(COUNT)).read_unaligned();
        if count == 0 {
            return 0;
        }
        let state = (lf_checker_rt::global::<u32>(STATE_PTR)).read_unaligned();
        let table = lf_checker_rt::relocated(TABLE);
        let mut i: u32 = 0;
        while i < count {
            let row = table + i * STRIDE;
            if (row as *const u32).read_unaligned() == key {
                let want = ((state + STATE_OFF) as *const u32).read_unaligned();
                if ((row + 8) as *const u32).read_unaligned() == want {
                    return 1;
                }
            }
            i += 1;
        }
        0
    }
});
