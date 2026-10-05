// original: 0x0093DF10 stream_node_step (proposed)

/// Insert each later element before the first when it sorts earlier.
///
/// Walks the pointer range `[begin, end)` from the second element: when
/// an element's key sorts strictly below the first element's key, the
/// run is shifted right by one slot through the mover and the element
/// lands first; otherwise the element is placed through the joiner with
/// the context. (The second argument slot is read but never used.)
/// Answers the last worker's answer, or the entry EAX (pinned to zero
/// by this contract) when the range holds fewer than two elements.
lf_checker_rt::export!(cdecl, rw_0093df10(begin: u32, end: u32, _unused: u32, ctx: u32) -> u32 {
    unsafe {
        const MOVER: u32 = 1;
        const JOINER: u32 = 2;
        const SLOT: u32 = 4;
        if begin == end {
            return 0;
        }
        let mut answer: u32 = 0;
        let mut cur = begin.wrapping_add(SLOT);
        while cur != end {
            let cand = (cur as *const u32).read_unaligned();
            let first = (begin as *const u32).read_unaligned();
            let ck = (cand as *const u32).read_unaligned();
            let fk = (first as *const u32).read_unaligned();
            if ck < fk {
                let len = cur.wrapping_sub(begin);
                let r: u32 = lf_checker_rt::callee_cdecl!(
                    MOVER,
                    u32,
                    begin.wrapping_add(SLOT),
                    begin,
                    len
                );
                (begin as *mut u32).write_unaligned(cand);
                answer = r;
            } else {
                let r: u32 =
                    lf_checker_rt::callee_cdecl!(JOINER, u32, cur, cand, ctx);
                answer = r;
            }
            cur = cur.wrapping_add(SLOT);
        }
        answer
    }
});
