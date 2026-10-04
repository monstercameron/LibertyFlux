// original: 0x009e4ab0 task_kind_lookup (proposed)

/// Map a kind word through a 78-entry 0/1 table, default 1.
///
/// Reads the word at `p`, subtracts 0x12d and, when the result exceeds
/// 0x4d, returns 1. Otherwise the original dispatches through a
/// two-target jump table whose arms return 0 and 1, so the result
/// equals the table byte; the table holds 0 only at indices 0, 2, 4, 6
/// and 77. The table sits in the code section, which the checker
/// revokes while the rewrite runs, so the decision is written out
/// rather than read from the image. Only `al` is defined on return.
/// `cdecl`, one stack word.
lf_checker_rt::export!(cdecl, rw_009e4ab0(p: u32) -> u32 {
    unsafe {
        const BIAS: u32 = 0x12d;
        const LIMIT: u32 = 0x4d;
        let v = ((p) as *const u32).read_unaligned().wrapping_sub(BIAS);
        if v > LIMIT {
            return 1;
        }
        match v {
            0 | 2 | 4 | 6 | 77 => 0,
            _ => 1,
        }
    }
});
