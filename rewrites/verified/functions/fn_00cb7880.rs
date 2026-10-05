// original: 0x00cb7880 indexed_entry_copy_7880
/// Copy one 16-byte table entry selected by the table's own count (leaf).
///
/// `this` (thiscall) points at an object whose word at `+0x64` is a table
/// pointer; the table's first word is a count `n`. When both are non-zero,
/// 16 bytes are copied from `table + n * 16` to `dst` and the last word
/// is returned with its low byte forced to 1. A null table returns 0; a
/// zero count returns the table address with its low byte cleared (the
/// original's `(an instruction of the original)` clears only the low byte). No calls.
lf_checker_rt::export!(thiscall, rw_00cb7880(this: u32, dst: u32) -> u32 {
    unsafe {
        /// Object word holding the table pointer.
        const TABLE_OFF: u32 = 0x64;
        /// Bytes per table entry; the count scales by a 4-bit shift.
        const ENTRY_SHIFT: u32 = 4;
        /// Bytes copied per entry.
        const ENTRY_LEN: u32 = 16;
        let table = ((this + TABLE_OFF) as *const u32).read_unaligned();
        if table == 0 {
            return 0;
        }
        let n = (table as *const u32).read_unaligned();
        if n == 0 {
            return table & 0xFFFFFF00;
        }
        let src = table.wrapping_add(n.wrapping_shl(ENTRY_SHIFT));
        let w0 = (src as *const u32).read_unaligned();
        let w1 = ((src + 4) as *const u32).read_unaligned();
        let w2 = ((src + 8) as *const u32).read_unaligned();
        let w3 = ((src + 12) as *const u32).read_unaligned();
        (dst as *mut u32).write_unaligned(w0);
        ((dst + 4) as *mut u32).write_unaligned(w1);
        ((dst + 8) as *mut u32).write_unaligned(w2);
        ((dst + 12) as *mut u32).write_unaligned(w3);
        (w3 & 0xFFFFFF00) | 1
    }
});
