// original: 0x0099D960 audio_heapify_and_compact (proposed)

/// Sift small-keyed records through the heap base, then compact downwards.
///
/// The preamble (callee 1, cdecl/3 of the base, the low bound and the flag;
/// its answer is ignored) runs first. Then every record in [`lo`, `hi`) is
/// scanned with UNSIGNED pointer and key comparisons: a record whose key
/// (word at +12) is strictly below the base record's key is overwritten by
/// the base record's 16 bytes, and its old contents travel by value to the
/// sift-down (callee 2, cdecl/8 of the base, `(lo - base) / 16`, zero, the
/// four words and the flag). Finally the window compacts downwards: while
/// more than one record stands between the base and the low bound
/// (`((q - base) & ~0xF) > 0x10`, SIGNED), the compactor (callee 3, cdecl/3
/// of the base, the cursor and a flag word built from the low byte of the
/// incoming flag over the low bound) runs and the cursor steps down one
/// record. The original
/// writes the flag word back over its own incoming low-bound slot, so the
/// contract disables the stack check; the word is observed as the
/// compactor's third argument instead. Returns the leftover span. The
/// fourth word is padding the original never reads.
///
/// Original: cdecl of five words (base, lo, hi, pad, flag); the fourth word
/// is padding the original never reads.
lf_checker_rt::export!(cdecl, rw_0099D960(base: u32, lo: u32, hi: u32, _pad: u32, flag: u32) -> u32 {
    unsafe {
        const REC: u32 = 16;
        const KEY: u32 = 12;
        const PRE_CALLEE: u32 = 1;
        const SIFT_CALLEE: u32 = 2;
        const COMPACT_CALLEE: u32 = 3;
        lf_checker_rt::callee_cdecl!(PRE_CALLEE, u32, base, lo, flag);
        // Unsigned pointer comparison, as the original's `jae`.
        let mut p = lo;
        while p < hi {
            let pk = ((p.wrapping_add(KEY)) as *const u32).read_unaligned();
            let bk = ((base.wrapping_add(KEY)) as *const u32).read_unaligned();
            // Unsigned key comparison, as the original's `jae`.
            if pk < bk {
                let s = p as *mut u32;
                let b = base as *const u32;
                let old = [*(p as *const u32), *((p.wrapping_add(4)) as *const u32),
                           *((p.wrapping_add(8)) as *const u32), *((p.wrapping_add(12)) as *const u32)];
                *s = *b;
                *s.add(1) = *b.add(1);
                *s.add(2) = *b.add(2);
                *s.add(3) = *b.add(3);
                let idx = lo.wrapping_sub(base) >> 4;
                lf_checker_rt::callee_cdecl!(SIFT_CALLEE, u32, base, 0, idx,
                                             old[0], old[1], old[2], old[3], flag);
            }
            p = p.wrapping_add(REC);
        }
        let fword = (lo & 0xFFFF_FF00) | (flag & 0xFF);
        let mut left = lo.wrapping_sub(base);
        let mut q = lo;
        // Signed span comparison, as the original's `jle`/`jg`.
        while ((left & !(REC - 1)) as i32) > REC as i32 {
            lf_checker_rt::callee_cdecl!(COMPACT_CALLEE, u32, base, q, fword);
            left = left.wrapping_sub(REC);
            q = q.wrapping_sub(REC);
        }
        left & !(REC - 1)
    }
});
