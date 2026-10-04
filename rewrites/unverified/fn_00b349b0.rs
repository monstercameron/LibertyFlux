// original: 0x00b349b0 sort_build_runs_16b (proposed)

/// Build the sort runs over the 16-byte elements in [`first`, `last`): with
/// element count `(last - first) / 16` (signed), call the run routine for
/// indexes `(count - 2) / 2` down to 0, passing `first`, the index, the
/// count, the indexed element's 16 bytes by value, and the context word. The
/// checked contract always passes at least two elements (a shorter range
/// would return the caller's entry register, which no rewrite can observe);
/// the result is the last run call's answer. Original: 0x00b349b0 (cdecl,
/// five stack words: first, last, context, two unread).
lf_checker_rt::export!(cdecl, rw_00b349b0(
    first: u32,
    last: u32,
    ctx: u32,
    _u1: u32,
    _u2: u32,
) -> u32 {
    unsafe {
        const RUN: u32 = 1;
        const ELEM: u32 = 16;
        let count =
            (last.wrapping_sub(first) as i32 / ELEM as i32) as u32;
        let mut answer = 0u32;
        let mut index = ((count as i32 - 2) / 2) as u32;
        loop {
            let elem = first.wrapping_add(index.wrapping_mul(ELEM));
            let p = elem as *const u32;
            answer = lf_checker_rt::callee_cdecl!(
                RUN,
                u32,
                first,
                index,
                count,
                p.read_unaligned(),
                p.add(1).read_unaligned(),
                p.add(2).read_unaligned(),
                p.add(3).read_unaligned(),
                ctx
            );
            if index == 0 {
                break;
            }
            index -= 1;
        }
        answer
    }
});
