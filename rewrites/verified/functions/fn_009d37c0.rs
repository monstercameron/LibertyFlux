// original: 0x009D37C0 tagged_row_search (proposed)
//
/// Searches a global object table for the first row tagged with a value.
///
/// Reads the entry count (a 16-bit word, zero-extended, so always >= 0; the
/// loop bound compares that follow are signed but never see a negative) and
/// the entry array from globals. For each entry, a signed 16-bit slot index
/// at `+0x2e` selects a row through a second global table, and the row's tag
/// dword at `+0x3c` is compared with `want`. Returns the first matching entry
/// or null. Cdecl, one stack word.
lf_checker_rt::export!(cdecl, rw_009D37C0(want: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x011A9D1C;
        const ENTRIES: u32 = 0x011A9D18;
        const ROWS: u32 = 0x01295CD8;
        const SLOT: u32 = 0x2e;
        const TAG: u32 = 0x3c;
        #[inline(always)]
        unsafe fn rd(a: u32) -> u32 { unsafe { (a as *const u32).read_unaligned() } }
        let count = lf_checker_rt::global::<u16>(COUNT).read() as u32;
        let entries = lf_checker_rt::global::<u32>(ENTRIES).read();
        if (count as i32) <= 0 {
            return 0;
        }
        let mut i = 0u32;
        loop {
            let entry = rd(entries.wrapping_add(i.wrapping_mul(4)));
            let idx = (entry.wrapping_add(SLOT) as *const i16).read_unaligned() as i32;
            let rows = lf_checker_rt::relocated(ROWS);
            let row = rd(rows.wrapping_add((idx as u32).wrapping_mul(4)));
            if rd(row.wrapping_add(TAG)) == want {
                return entry;
            }
            i += 1;
            if (i as i32) >= (count as i32) {
                return 0;
            }
        }
    }
});
