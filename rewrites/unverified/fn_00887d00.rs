// original: 0x00887D00 stream_best_slot (proposed)

/// Find the loaded slot nearest above the caller's mark.
///
/// `mark` is the threshold and `out` the result slot: the slot is first
/// set to -1, then every row of the slot table (0xa0 bytes per row, count
/// and base from globals) whose value at `+0x2c` lies strictly above the
/// mark and strictly below the best value so far becomes the new best. The
/// answer is the best row, or 0 when the table is empty or no row qualifies.
///
/// Original: 0x00887D00 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00887D00(mark: u32, out: u32) -> u32 {
    unsafe {
        const COUNT_GLOBAL: u32 = 0x0115_a468;
        const TABLE_GLOBAL: u32 = 0x0115_a46c;
        const ROW: u32 = 0xa0;
        const VAL_OFF: u32 = 0x2c;
        const NONE: u32 = 0xffff_ffff;
        (out as *mut u32).write_unaligned(NONE);
        let count =
            (lf_checker_rt::relocated(COUNT_GLOBAL) as *const u32).read_unaligned();
        if count == 0 {
            return 0;
        }
        let base =
            (lf_checker_rt::relocated(TABLE_GLOBAL) as *const u32).read_unaligned();
        let mut best = 0u32;
        let mut i = 0u32;
        while i < count {
            if i != NONE {
                let e = base.wrapping_add(i.wrapping_mul(5).wrapping_mul(32));
                if e != 0 {
                    let v = ((e + VAL_OFF) as *const u32).read_unaligned();
                    let cur = (out as *const u32).read_unaligned();
                    if v > mark && v < cur {
                        (out as *mut u32).write_unaligned(v);
                        best = e;
                    }
                }
            }
            i = i.wrapping_add(1);
        }
        best
    }
});
