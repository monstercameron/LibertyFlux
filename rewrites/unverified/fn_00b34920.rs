// original: 0x00b34920 sort_build_runs_key0 (proposed)

/// Build the sort runs over the 28-byte elements in [`first`, `last`): with
/// element count `(last - first) / 28` (signed), call the leading-key run
/// routine for indexes `(count - 2) / 2` down to 0, passing `first`, the
/// index, the count, the indexed element's 28 bytes by value, and the
/// context word. With fewer than two elements nothing is called and the
/// result is the low word of the count-division's multiply step. Otherwise
/// the result is the last run call's answer. Original: 0x00b34920 (cdecl,
/// five stack words: first, last, context, two unread).
lf_checker_rt::export!(cdecl, rw_00b34920(
    first: u32,
    last: u32,
    ctx: u32,
    _u1: u32,
    _u2: u32,
) -> u32 {
    unsafe {
        const RUN: u32 = 1;
        const ELEM: u32 = 28;
        const DIV_MAGIC: u32 = 0x9249_2493;
        let span = last.wrapping_sub(first);
        let count = (span as i32 / ELEM as i32) as u32;
        if (count as i32) < 2 {
            return DIV_MAGIC.wrapping_mul(span);
        }
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
                p.add(4).read_unaligned(),
                p.add(5).read_unaligned(),
                p.add(6).read_unaligned(),
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
