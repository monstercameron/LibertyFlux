// original: 0x00b350e0 insertion_pass_16b (proposed)

/// Run one insertion pass over the 16-byte elements in [`first`, `last`):
/// for each element, call the insertion step with the element's address and
/// its 16 bytes, plus the `fill` word as a trailing argument the step reads
/// past its value (the step only consumes the element bytes). The range is
/// never empty in the checked contract (an empty range would return the
/// caller's entry register, which no rewrite can observe); the result is the
/// last step's answer. Original: 0x00b350e0 (cdecl, three stack words:
/// first, last, fill).
lf_checker_rt::export!(cdecl, rw_00b350e0(first: u32, last: u32, fill: u32) -> u32 {
    unsafe {
        const STEP: u32 = 1;
        const ELEM: u32 = 16;
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
                fill
            );
            elem = elem.wrapping_add(ELEM);
        }
        answer
    }
});
