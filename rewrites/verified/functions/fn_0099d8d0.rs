// original: 0x0099D8D0 audio_build_max_heap (proposed)

/// Heapify 16-byte records into a max-heap by the key at +12.
///
/// The count `(end - begin) / 16` is SIGNED: fewer than two records returns
/// at once (the original falls through with the entry `eax`, which the
/// contract fixes to zero). Otherwise the sift-down (callee 1, cdecl/8 of
/// the base, the index, the count, the four words of the record at that
/// index and the flag) runs for every index from `(count - 2) / 2` down to
/// zero, each record passed by value from the array. Returns the last
/// sift-down's answer. Plain cdecl of three words.
lf_checker_rt::export!(cdecl, rw_0099D8D0(begin: u32, end: u32, flag: u32) -> u32 {
    unsafe {
        const REC: u32 = 16;
        const SIFT_CALLEE: u32 = 1;
        // Signed shift, as the original's `sar`.
        let count = (end.wrapping_sub(begin) as i32) >> 4;
        if count < 2 {
            return 0; // Entry eax, fixed to zero by the contract.
        }
        // Signed division, as the original's `cdq`/`sub`/`sar` sequence.
        let top = (count - 2) / 2;
        let mut ans: u32 = 0;
        let mut idx = top;
        loop {
            let rec = begin.wrapping_add((idx as u32).wrapping_mul(REC)) as *const u32;
            ans = lf_checker_rt::callee_cdecl!(SIFT_CALLEE, u32, begin, idx as u32,
                                               count as u32, rec.read_unaligned(),
                                               rec.add(1).read_unaligned(),
                                               rec.add(2).read_unaligned(),
                                               rec.add(3).read_unaligned(), flag);
            if idx == 0 {
                break;
            }
            idx -= 1;
        }
        ans
    }
});
