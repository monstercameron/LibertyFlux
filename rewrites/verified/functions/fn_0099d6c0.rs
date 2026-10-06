// original: 0x0099D6C0 audio_finish_record_sort (proposed)

/// Finish sorting 16-byte records by key, splitting large ranges in two.
///
/// The span `(hi - lo) & ~0xF` is compared SIGNED against 0x100. Small spans
/// go straight to the insertion pass (callee 1, cdecl/4 of the bounds, a
/// zero word and the flag). Large spans sort the first 0x100 bytes with the
/// insertion pass and feed the rest to the per-record inserter (callee 2,
/// cdecl/4 of the same shape). Returns the last helper's answer on each
/// path, as the original falls through with `eax` holding it. Plain cdecl
/// of three words.
lf_checker_rt::export!(cdecl, rw_0099D6C0(lo: u32, hi: u32, flag: u32) -> u32 {
    unsafe {
        const REC: u32 = 16;
        const SPLIT: u32 = 0x100;
        const INSERT_CALLEE: u32 = 1;
        const EACH_CALLEE: u32 = 2;
        let span = hi.wrapping_sub(lo) & !(REC - 1);
        // Signed comparison, as the original's `jle`.
        if (span as i32) > SPLIT as i32 {
            let mid = lo.wrapping_add(SPLIT);
            lf_checker_rt::callee_cdecl!(INSERT_CALLEE, u32, lo, mid, 0, flag);
            lf_checker_rt::callee_cdecl!(EACH_CALLEE, u32, mid, hi, 0, flag)
        } else {
            lf_checker_rt::callee_cdecl!(INSERT_CALLEE, u32, lo, hi, 0, flag)
        }
    }
});
