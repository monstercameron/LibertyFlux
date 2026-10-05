// original: 0x00AED300 kv_reinsert_range (proposed)

/// Reinsert every entry of a range through the insertion step.
///
/// `first` and `last` delimit 8-byte entries; for each entry in turn the
/// insertion-step callee is called with the entry address, its key and
/// value words, and `ctx`. The third stack word is read by no instruction.
/// Returns nothing meaningful (eax holds the last callee result, or entry
/// garbage when the range is empty). Callers pass ranges whose length is a
/// multiple of 8.
///
/// Original: 0x00AED300 (cdecl, four stack words, one direct callee).
lf_checker_rt::export!(cdecl, rw_00aed300(first: u32, last: u32, _u: u32, ctx: u32) -> () {
    unsafe {
        const ENTRY: u32 = 8;
        const STEP_CALLEE: u32 = 1;
        let mut elem = first;
        while elem != last {
            let key = ((elem) as *const u32).read_unaligned();
            let val = ((elem.wrapping_add(4)) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(STEP_CALLEE, u32, elem, key, val, ctx);
            elem = elem.wrapping_add(ENTRY);
        }
    }
});
