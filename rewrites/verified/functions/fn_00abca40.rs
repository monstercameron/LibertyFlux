// original: 0x00abca40 capped_table_row_append (proposed)

/// Append one 80-byte row to a global table, unless the table is full.
///
/// The table base pointer lives in a global word, the row count in another.
/// The next slot is `count + 1`; when it reaches 63 (compared signed) the
/// function stores nothing and returns the unchanged count. Otherwise it
/// stores the new count, copies four 16-byte chunks from the four source
/// pointers into row offsets `0x00`, `0x10`, `0x20`, `0x30`, stores the four
/// tail words at `0x40`, `0x44`, `0x48`, `0x4c`, and records the new count in
/// a global index table at `INDEX_TABLE + key * 4`, where `key` is a third
/// global word. Rows are 80 bytes (`count * 10 * 8`, wrapping).
///
/// All copies are bitwise: the middle words of each chunk travel through
/// vector registers in the original but no arithmetic is done on them, so any
/// bit pattern round-trips unchanged.
///
/// Original: 0x00abca40 (cdecl, eight stack words: four source pointers, three
/// tail words, one tail word; returns the old count when full, else `key`).
lf_checker_rt::export!(cdecl, rw_00abca40(src0: u32, src1: u32, src2: u32, src3: u32, tail0: u32, tail1: u32, tail2: u32, tail3: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x150E248;
        const KEY: u32 = 0x150E250;
        const TABLE_PTR: u32 = 0x103EEE8;
        const INDEX_TABLE: u32 = 0x154DFE0;
        const CAP_NEXT: i32 = 63;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let count = rd32(lf_checker_rt::relocated(COUNT));
        let next = count.wrapping_add(1);
        if (next as i32) >= CAP_NEXT {
            return count;
        }
        wr32(lf_checker_rt::relocated(COUNT), next);
        let base = rd32(lf_checker_rt::relocated(TABLE_PTR));
        let row = base.wrapping_add(count.wrapping_mul(10).wrapping_mul(8));
        wr32(row.wrapping_add(0x4c), tail3);
        let srcs = [src0, src1, src2, src3];
        let mut off = 0u32;
        let mut k = 0usize;
        while k < 4 {
            let s = srcs[k];
            wr32(row.wrapping_add(off), rd32(s));
            wr32(row.wrapping_add(off + 4), rd32(s.wrapping_add(4)));
            wr32(row.wrapping_add(off + 8), rd32(s.wrapping_add(8)));
            wr32(row.wrapping_add(off + 12), rd32(s.wrapping_add(12)));
            off += 16;
            k += 1;
        }
        wr32(row.wrapping_add(0x40), tail0);
        wr32(row.wrapping_add(0x44), tail1);
        wr32(row.wrapping_add(0x48), tail2);
        let key = rd32(lf_checker_rt::relocated(KEY));
        wr32(
            lf_checker_rt::relocated(INDEX_TABLE).wrapping_add(key.wrapping_mul(4)),
            next,
        );
        key
    }
});
