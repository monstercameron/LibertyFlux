// original: 0x008E7970 table_stamp_u16_slots (proposed)

/// Stamp a constant 16-bit slot value into every entry of a 64-row table.
///
/// `obj` points to an object holding two parallel 64-entry tables: row base
/// pointers at `ROW_PTRS` and signed row counts at `ROW_COUNTS`. For each
/// row whose base is non-null and whose count is positive (signed), the
/// 16-bit slot at `SLOT_OFF` inside every `ENTRY_STRIDE`-byte entry is set
/// to `STAMP`.
///
/// Returns `obj + ROW_COUNTS + 64 * 4`: the table cursor the original leaves
/// in `eax` after the 64th row.
///
/// Original: 0x008E7970 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_008E7970(obj: u32) -> u32 {
    unsafe {
        /// Offset of the 64 row-base pointers.
        const ROW_PTRS: u32 = 0x804;
        /// Offset of the 64 signed row counts.
        const ROW_COUNTS: u32 = 0xB04;
        /// Number of rows.
        const ROWS: u32 = 64;
        /// Size of one row entry in bytes.
        const ENTRY_STRIDE: u32 = 0x20;
        /// Offset of the stamped slot inside an entry.
        const SLOT_OFF: u32 = 0x10;
        /// Stamped slot value.
        const STAMP: u16 = 0x7FFE;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
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
                        .wrapping_add(SLOT_OFF);
                    wr16(a, STAMP);
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
        obj.wrapping_add(ROW_COUNTS).wrapping_add(ROWS.wrapping_mul(4))
    }
});
