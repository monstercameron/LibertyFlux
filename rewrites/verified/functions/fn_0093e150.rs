// original: 0x0093e150 ptr_insertion_sort (proposed)

/// Insertion-sort the pointer range [`begin`, `end`) via the step callee.
///
/// Calls the step callee once per slot with (slot, contents, `extra`).
/// Note the quirk: `extra` is the fourth stack word; the third is never
/// read. An empty range makes no calls. The return value is the last
/// callee answer, or entry garbage for an empty range, so it is not
/// compared.
///
/// Original: 0x0093e150 (cdecl, four stack words; one direct callee).
lf_checker_rt::export!(cdecl, rw_0093e150(begin: u32, end: u32, _pad: u32, extra: u32) -> u32 {
    const STEP: u32 = 1;
    unsafe {
        let mut slot = begin;
        while slot != end {
            let val = (slot as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(STEP, u32, slot, val, extra);
            slot = slot.wrapping_add(4);
        }
        0
    }
});
