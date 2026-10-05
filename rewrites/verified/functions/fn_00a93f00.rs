// original: 0x00a93f00 stream_swap_entry_links (proposed)

/// Swap the link words of two stream entries and re-point the table.
///
/// Both arguments point at 24-byte entries. Each entry's slot number is
/// `(entry - table_base) / 24` (signed, magic multiply by `0x2aaaaaab`),
/// truncated to 16 bits. The word at `+0x10` of the second entry moves to
/// the first entry's `+0x10`, the first entry's `+0x12` takes the second
/// entry's slot, the second entry's `+0x10` takes the first entry's slot,
/// and the table entry selected by the moved word gets the first entry's
/// slot at its `+0x12`. Returns the table base.
///
/// Original: thiscall, one stack argument (second entry).
lf_checker_rt::export!(thiscall, rw_00a93f00(first: u32, second: u32) -> u32 {
    unsafe {
        const TABLE_BASE_G: u32 = 0x012fb3a8;
        const ENT_LINK: u32 = 0x10;
        const ENT_SLOT: u32 = 0x12;
        const ENTRY_STRIDE: u32 = 24;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd16w(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        /// Signed divide by 24 as the original's magic sequence does it.
        fn div24(d: i32) -> i32 {
            let hi = ((d as i64 * 0x2aaaaaabi64) >> 32) as i32;
            let s = hi >> 2;
            s.wrapping_add(((s as u32) >> 31) as i32)
        }
        let base = rd32(lf_checker_rt::relocated(TABLE_BASE_G));
        let slot_first =
            div24((first as i32).wrapping_sub(base as i32)) as u16;
        let slot_second =
            div24((second as i32).wrapping_sub(base as i32)) as u16;
        let moved = rd16w(second.wrapping_add(ENT_LINK));
        wr16(first.wrapping_add(ENT_LINK), moved);
        wr16(first.wrapping_add(ENT_SLOT), slot_second);
        wr16(second.wrapping_add(ENT_LINK), slot_first);
        let target =
            base.wrapping_add((moved as u32).wrapping_mul(ENTRY_STRIDE));
        wr16(target.wrapping_add(ENT_SLOT), slot_first);
        base
    }
});
