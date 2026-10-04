// original: 0x00b77d90 slot_table_get (proposed)

/// Return one of the three slot-table pointers by index.
///
/// Returns the pointer stored at `index` in the slot table, or 0 when the
/// index is out of range (3 or above, unsigned comparison).
///
/// Original: cdecl with one stack word, plain `ret`.
lf_checker_rt::export!(cdecl, rw_00b77d90(index: u32) -> u32 {
    unsafe {
        const SLOT_TABLE: u32 = 0x0167CCC4;
        const SLOTS: u32 = 3;
        if index >= SLOTS {
            return 0;
        }
        let base = lf_checker_rt::relocated(SLOT_TABLE);
        ((base + index * 4) as *const u32).read_unaligned()
    }
});
