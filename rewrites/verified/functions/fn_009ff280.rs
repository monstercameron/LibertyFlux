// original: 0x009FF280 frag_triple_table_append (proposed)

/// Append one three-word row to the global frag triple table.
///
/// The table starts at a fixed global address with a dword count at another
/// fixed global; each row is three dwords (12 bytes). When the count has
/// reached `CAPACITY` (1024) the call does nothing. Otherwise the three
/// argument words are stored at row `count` and the count is incremented.
/// Returns nothing meaningful (the original leaves the third argument, or
/// the incoming `eax` on the full path, in `eax`).
///
/// Original: 0x009FF280 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_009FF280(w0: u32, w1: u32, w2: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x012BCC90;
        const TABLE: u32 = 0x012B9C90;
        const STRIDE: u32 = 12;
        const CAPACITY: u32 = 0x400;
        let count_ptr = lf_checker_rt::global::<u32>(COUNT);
        let count = count_ptr.read_unaligned();
        if count >= CAPACITY {
            return 0;
        }
        let row = lf_checker_rt::relocated(TABLE) + count * STRIDE;
        ((row) as *mut u32).write_unaligned(w0);
        ((row + 4) as *mut u32).write_unaligned(w1);
        ((row + 8) as *mut u32).write_unaligned(w2);
        count_ptr.write_unaligned(count + 1);
        0
    }
});
