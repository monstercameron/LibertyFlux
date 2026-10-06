// original: 0x008EF310 table_clear_flag_bits (proposed)

/// Clear bit 4 of the flag byte of every entry in a 64-row table.
///
/// `obj` points to an object holding two parallel 64-entry tables: row base
/// pointers at `ROW_PTRS` and signed row counts at `ROW_COUNTS`. For each
/// row whose base is non-null and whose count is positive (signed), the byte
/// at `ENTRY_FLAG` inside every `ENTRY_STRIDE`-byte entry has `FLAG_KEEP`
/// applied (bit 4 cleared, all other bits kept).
///
/// Returns `obj + ROW_COUNTS + 64 * 4`: the table cursor the original leaves
/// in `eax` after the 64th row.
///
/// Original: 0x008EF310 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_008EF310(obj: u32) -> u32 {
    unsafe {
        /// Offset of the 64 row-base pointers.
        const ROW_PTRS: u32 = 0x804;
        /// Offset of the 64 signed row counts.
        const ROW_COUNTS: u32 = 0xB04;
        /// Number of rows.
        const ROWS: u32 = 64;
        /// Size of one row entry in bytes.
        const ENTRY_STRIDE: u32 = 0x20;
        /// Offset of the flag byte inside an entry.
        const ENTRY_FLAG: u32 = 0x1F;
        /// Kept bits of the flag byte (bit 4 cleared).
        const FLAG_KEEP: u8 = 0xEF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let mut i: u32 = 0;
        while i < ROWS {
            let slot = i.wrapping_mul(4);
            let row = rd32(obj.wrapping_add(ROW_PTRS).wrapping_add(slot));
            let count = rd32(obj.wrapping_add(ROW_COUNTS).wrapping_add(slot)) as i32;
            if row != 0 && count > 0 {
                let mut j: u32 = 0;
                while j < count as u32 {
                    let a = row
                        .wrapping_add(j.wrapping_mul(ENTRY_STRIDE))
                        .wrapping_add(ENTRY_FLAG);
                    wr8(a, rd8(a) & FLAG_KEEP);
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
        obj.wrapping_add(ROW_COUNTS).wrapping_add(ROWS.wrapping_mul(4))
    }
});
