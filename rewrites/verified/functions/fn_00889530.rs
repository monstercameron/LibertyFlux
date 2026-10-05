// original: 0x00889530 stream_record_find (proposed)

/// Find a record by id in the stream's sorted table, under its lock.
///
/// Locks (callee 1); when the table at `[this+0x3c]` is present and both
/// state bytes (`+0x46`/`+0x47`) are clear, locates the id's row with the
/// binary search entry (callee 2) and scans its variable-length records
/// (stride `word at +4` plus 8) for `arg1`, returning the record address.
/// Anything missing (no table, a state byte set, search miss, empty row,
/// record miss) unlocks (callee 3) and returns 0.
///
/// Original: 0x00889530 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00889530(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x3c;
        const LOCK_WORD: u32 = 0x38;
        const STATE0: u32 = 0x46;
        const STATE1: u32 = 0x47;
        const ROW_HI: u32 = 0x14;
        const ROW_TAB: u32 = 0x08;
        const LOCK: u32 = 1;
        const SEARCH: u32 = 2;
        const UNLOCK: u32 = 3;
        let w = ((this + LOCK_WORD) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LOCK, u32, w);
        let mut found = 0u32;
        let s = ((this + TABLE) as *const u32).read_unaligned();
        let s0 = ((this + STATE0) as *const u8).read();
        let s1 = ((this + STATE1) as *const u8).read();
        if s != 0 && s0 == 0 && s1 == 0 {
            let hi = ((s + ROW_HI) as *const u32).read_unaligned();
            let tab = ((s + ROW_TAB) as *const u32).read_unaligned();
            let idx = lf_checker_rt::callee_stdcall!(
                SEARCH, u32, arg0, tab, hi);
            if (idx as i32) >= 0 {
                let row = tab.wrapping_add((idx.wrapping_mul(2)).wrapping_mul(8));
                let len = ((row + 0x0c) as *const u32).read_unaligned();
                let base = ((row) as *const u32).read_unaligned();
                if len != 0 {
                    let mut off = 0u32;
                    while off < len {
                        let at = base.wrapping_add(off);
                        if ((at) as *const u32).read_unaligned() == arg1 {
                            found = at;
                            break;
                        }
                        let st = ((at.wrapping_add(4)) as *const u32)
                            .read_unaligned();
                        off = off.wrapping_add(st.wrapping_add(8));
                    }
                }
            }
        }
        lf_checker_rt::callee_cdecl!(UNLOCK, u32, w);
        found
    }
});
