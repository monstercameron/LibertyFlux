// original: 0x008EA8A0 count_flagged_rows_global (proposed)

/// Count the table rows whose flag nibble is clear (global tables).
///
/// Same shape as `count_flagged_rows`, except the descriptor fields live on
/// `obj` itself (key at `DESC_KEY`, signed start offset at `DESC_OFF`,
/// iteration-count byte at `DESC_ITERS`) and the two tables are globals:
/// row bases at `BASE_TABLE`, entry pointers at `ROW_TABLE`.
///
/// For each of the `iters` rows: read the row word, look up the entry
/// pointer by its low 16 bits, skip null entries; otherwise read the flag
/// byte at `FLAG_OFF` past the entry plus the high 16 bits scaled by
/// `HI_SCALE`, and count the row when the high nibble is clear. Returns
/// the count.
///
/// Original: 0x008EA8A0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_008EA8A0(obj: u32) -> u32 {
    unsafe {
        /// Global entry-pointer table (indexed by row word low 16).
        const ROW_TABLE: u32 = 0x1178284;
        /// Global row-base table (indexed by descriptor key).
        const BASE_TABLE: u32 = 0x1178384;
        /// Object offset of the unsigned 16-bit base key.
        const DESC_KEY: u32 = 8;
        /// Object offset of the signed 16-bit start offset.
        const DESC_OFF: u32 = 0x12;
        /// Object offset of the iteration-count byte.
        const DESC_ITERS: u32 = 0x1E;
        /// Low bits of the count byte that form the count.
        const ITERS_MASK: u8 = 0x0F;
        /// Row stride in bytes.
        const ROW_STRIDE: u32 = 8;
        /// Scale applied to the row word's high 16 bits.
        const HI_SCALE: u32 = 5;
        /// Offset of the flag byte past entry + scaled high bits.
        const FLAG_OFF: u32 = 0x1C;
        /// Tested flag nibble (row counted when these bits are clear).
        const FLAG_MASK: u8 = 0xF0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        let iters = u32::from(rd8(obj.wrapping_add(DESC_ITERS)) & ITERS_MASK);
        if iters == 0 {
            return 0;
        }
        let key = u32::from(rd16(obj.wrapping_add(DESC_KEY)));
        let off = rd16(obj.wrapping_add(DESC_OFF)) as i16 as i32;
        let base = rd32(
            lf_checker_rt::relocated(BASE_TABLE).wrapping_add(key.wrapping_mul(4)),
        );
        let mut cur = base.wrapping_add((off.wrapping_mul(ROW_STRIDE as i32)) as u32);
        let row_table = lf_checker_rt::relocated(ROW_TABLE);
        let mut n = iters;
        let mut count: u32 = 0;
        while n != 0 {
            let v = rd32(cur);
            let entry = rd32(row_table.wrapping_add((v & 0xFFFF).wrapping_mul(4)));
            if entry != 0 {
                let flag = rd8(
                    entry
                        .wrapping_add((v >> 16) << HI_SCALE)
                        .wrapping_add(FLAG_OFF),
                );
                if flag & FLAG_MASK == 0 {
                    count = count.wrapping_add(1);
                }
            }
            cur = cur.wrapping_add(ROW_STRIDE);
            n = n.wrapping_sub(1);
        }
        count
    }
});
