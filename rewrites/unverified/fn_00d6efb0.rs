// original: 0x00D6EFB0 CReplayProgressBar::count_entries_before_current_time

/// Counts consecutive entries in the pointer array at vector offset 0x9c whose unsigned word at entry offset 0x14 is less than the unsigned key at object offset 0xb8. The vector count is a 16-bit value and traversal stops at the first entry greater than or equal to the key.
lf_checker_rt::export!(thiscall, rw_00d6efb0(this: u32) -> u32 {
    unsafe {
        const VECTOR_FIELD: u32 = 0x9c;
        const KEY_FIELD: u32 = 0xb8;
        const VECTOR_DATA: u32 = 0;
        const VECTOR_COUNT: u32 = 4;
        const ENTRY_KEY: u32 = 0x14;
        let vector = (this.wrapping_add(VECTOR_FIELD) as *const u32).read_unaligned();
        let key = (this.wrapping_add(KEY_FIELD) as *const u32).read_unaligned();
        let entries = (vector.wrapping_add(VECTOR_DATA) as *const u32).read_unaligned();
        let count = (vector.wrapping_add(VECTOR_COUNT) as *const u16).read_unaligned() as u32;
        let mut before = 0u32;
        while before < count {
            let entry = (entries.wrapping_add(before * 4) as *const u32).read_unaligned();
            let entry_key = (entry.wrapping_add(ENTRY_KEY) as *const u32).read_unaligned();
            if entry_key >= key {
                break;
            }
            before += 1;
        }
        before
    }
});
