// original: 0x0099DC80 audio_sort_records (proposed)

/// Sort 16-byte records by key: introsort with a finishing pass.
///
/// An empty range returns at once (the original falls through with the
/// entry `eax`, which the contract fixes to zero). Otherwise the record
/// count `(hi - lo) / 16` sets the depth budget to twice its base-2
/// logarithm, the introsort (callee 1, cdecl/5 of the bounds, a zero word,
/// the budget and the flag) runs, and the finishing pass (callee 2,
/// cdecl/3) runs. Counts below one hang the original (the halving loop
/// never reaches one), so the contract only feeds ranges of at least one
/// record. Returns the finishing pass's answer. Plain cdecl of three words.
lf_checker_rt::export!(cdecl, rw_0099DC80(lo: u32, hi: u32, flag: u32) -> u32 {
    unsafe {
        const REC: u32 = 16;
        const INTRO_CALLEE: u32 = 1;
        const FINISH_CALLEE: u32 = 2;
        if lo == hi {
            return 0; // Entry eax, fixed to zero by the contract.
        }
        let mut count = hi.wrapping_sub(lo) >> 4;
        let mut log: u32 = 0;
        // Bit length minus one; the original halves until exactly one.
        while count != 1 {
            count >>= 1;
            log += 1;
        }
        lf_checker_rt::callee_cdecl!(INTRO_CALLEE, u32, lo, hi, 0, log * 2, flag);
        lf_checker_rt::callee_cdecl!(FINISH_CALLEE, u32, lo, hi, flag)
    }
});
