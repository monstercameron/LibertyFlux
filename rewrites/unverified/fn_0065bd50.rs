// original: 0x0065bd50 rage::DayLighting::DayLightingVarCache::vf0 (symbols)

/// Resolve four daylight-variable names to their positions in a variable table.
///
/// `holder + 0x18` points at the table, whose rows live at `[table + 8] + 0x0c`
/// with a 16-bit row count at `[table + 0x0c]` and a stride of `0x30` bytes. For
/// each of the four name ids the hash callee is called with (`id`, 0); the rows
/// are then scanned in order comparing first the word at `+4` then the word at
/// `+0` against the hash. The 1-based position of the first matching row, or 0
/// when no row matches (or the count is 0), is stored to `this + 4/8/0x0c/0x10`.
/// The return value is the fourth lookup's outcome, except that a miss there
/// returns the row count rather than 0.
///
/// Original: 0x0065bd50 (thiscall: `this` in ECX, `holder` the one stack word,
/// callee pops 4). The name ids are pointers to names in the original's
/// read-only data (all four carry relocations), so they are resolved through
/// `relocated` for the worker's mapping. No floating point, no globals.
lf_checker_rt::export!(thiscall, rw_0065bd50(this: u32, holder: u32) -> u32 {
    unsafe {
        const HASH_CALLEE: u32 = 1;
        const NAME_IDS: [u32; 4] = [0x00FE2F1C, 0x00FE2EE8, 0x00FE2EF8, 0x00FE2F2C];
        const INDEX_SLOTS: [u32; 4] = [0x04, 0x08, 0x0C, 0x10];
        const HOLDER_TABLE: u32 = 0x18;
        const TABLE_ROWS: u32 = 0x08;
        const TABLE_COUNT: u32 = 0x0C;
        const ROW_STRIDE: u32 = 0x30;
        const ROW_HEADER: u32 = 0x0C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let table = rd32(holder + HOLDER_TABLE);
        let mut last = 0u32;
        let mut i = 0usize;
        while i < 4 {
            let h = lf_checker_rt::callee_cdecl!(HASH_CALLEE, u32, lf_checker_rt::relocated(NAME_IDS[i]), 0u32);
            let rows = rd32(table + TABLE_ROWS) + ROW_HEADER;
            let count = rd16(table + TABLE_COUNT);
            let mut found = 0u32;
            let mut j = 0u32;
            while j < count {
                let row = rows + j * ROW_STRIDE;
                if rd32(row + 4) == h || rd32(row) == h {
                    found = j + 1;
                    break;
                }
                j += 1;
            }
            wr32(this + INDEX_SLOTS[i], found);
            last = if found != 0 { found } else { count };
            i += 1;
        }
        last
    }
});
