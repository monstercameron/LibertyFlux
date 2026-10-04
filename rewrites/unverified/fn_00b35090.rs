// original: 0x00b35090 insertion_pass_28b_key0 (proposed)

/// Run one insertion pass over the 28-byte elements in [`first`, `last`):
/// for each element, call the leading-key insertion step with the element's
/// address, its first 24 bytes, and the `fill` word in place of the last
/// value word. The range is never empty in the checked contract (an empty
/// range would return the caller's entry register, which no rewrite can
/// observe); the result is the last step's answer. Original: 0x00b35090
/// (cdecl, four stack words: first, last, an unread pad word, fill).
lf_checker_rt::export!(cdecl, rw_00b35090(first: u32, last: u32, _pad: u32, fill: u32) -> u32 {
    unsafe {
        const STEP: u32 = 1;
        const ELEM: u32 = 28;
        let mut answer = 0u32;
        let mut elem = first;
        while elem != last {
            let p = elem as *const u32;
            answer = lf_checker_rt::callee_cdecl!(
                STEP,
                u32,
                elem,
                p.read_unaligned(),
                p.add(1).read_unaligned(),
                p.add(2).read_unaligned(),
                p.add(3).read_unaligned(),
                p.add(4).read_unaligned(),
                p.add(5).read_unaligned(),
                fill
            );
            elem = elem.wrapping_add(ELEM);
        }
        answer
    }
});
