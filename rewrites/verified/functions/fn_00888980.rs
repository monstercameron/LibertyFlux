// original: 0x00888980 stream_table_free (proposed)

/// Free every chain of the stream table and the table itself.
///
/// Walks the `count` buckets (`[this]` table, word count at `+4`), freeing
/// (callee 1) each node of every chain (link at node `+8`), then frees the
/// table, zeroes the table word and the count dword, and returns 0.
///
/// Original: 0x00888980 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00888980(this: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x00;
        const COUNT: u32 = 0x04;
        const NEXT: u32 = 0x08;
        const FREE: u32 = 1;
        let count = ((this + COUNT) as *const u16).read_unaligned() as u32;
        let table = ((this + TABLE) as *const u32).read_unaligned();
        let mut i = 0u32;
        while i < count {
            let mut e = ((table.wrapping_add(i.wrapping_mul(4))) as *const u32)
                .read_unaligned();
            while e != 0 {
                let next = ((e + NEXT) as *const u32).read_unaligned();
                lf_checker_rt::callee_cdecl!(FREE, u32, e);
                e = next;
            }
            i = i.wrapping_add(1);
        }
        lf_checker_rt::callee_cdecl!(FREE, u32, table);
        ((this + TABLE) as *mut u32).write_unaligned(0);
        ((this + COUNT) as *mut u32).write_unaligned(0);
        0
    }
});
