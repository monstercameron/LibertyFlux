// original: 0x008B9020 frontend_table_pair_lookup

/// Copy one eight-byte table entry to the caller's buffer, or zeroes.
///
/// `out_ptr` receives the entry: when the table pointer `TABLE` is null the
/// buffer gets two zero words, otherwise it gets the two words of entry
/// `index` (each entry is eight bytes). Returns `out_ptr`. Cdecl, two
/// stack words.
lf_checker_rt::export!(cdecl, rw_008B9020(out_ptr: u32, index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0116_09C8;
        const ENTRY_BYTES: u32 = 8;
        let table = (lf_checker_rt::relocated(TABLE) as *const u32).read_unaligned();
        if table == 0 {
            (out_ptr as *mut u32).write_unaligned(0);
            (out_ptr.wrapping_add(4) as *mut u32).write_unaligned(0);
        } else {
            let entry = table.wrapping_add(index.wrapping_mul(ENTRY_BYTES));
            let lo = (entry as *const u32).read_unaligned();
            let hi = (entry.wrapping_add(4) as *const u32).read_unaligned();
            (out_ptr as *mut u32).write_unaligned(lo);
            (out_ptr.wrapping_add(4) as *mut u32).write_unaligned(hi);
        }
        out_ptr
    }
});
