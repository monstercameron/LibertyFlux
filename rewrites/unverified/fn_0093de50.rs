// original: 0x0093DE50 stream_chain_call (proposed)

/// Sift the hole down the pointer heap, then settle it.
///
/// From `start`, repeatedly moves the smaller-keyed child (right child
/// wins ties) into the hole while both children exist inside `count`;
/// a lone last left child moves once more when the hole's doubled index
/// plus two exactly meets `count`. Hands the array, the final hole, the
/// start, the context and the trailing argument to the settler and answers
/// its answer.
lf_checker_rt::export!(cdecl, rw_0093de50(arr: u32, start: u32, count: u32, ctx: u32, extra: u32) -> u32 {
    unsafe {
        const SETTLE: u32 = 1;
        const SLOT: u32 = 4;
        // NOTE: the key is the dword stored at the record, one step.
        let key = |p: u32| -> u32 {
            (p as *const u32).read_unaligned()
        };
        let at = |i: u32| -> u32 {
            ((arr.wrapping_add(i.wrapping_mul(SLOT))) as *const u32).read_unaligned()
        };
        let mut hole = start;
        let mut child = hole.wrapping_mul(2).wrapping_add(2);
        if child < count {
            loop {
                let right = at(child);
                let left = at(child.wrapping_sub(1));
                if key(right) < key(left) {
                    child = child.wrapping_sub(1);
                }
                (arr.wrapping_add(hole.wrapping_mul(SLOT)) as *mut u32)
                    .write_unaligned(at(child));
                hole = child;
                child = hole.wrapping_mul(2).wrapping_add(2);
                if child >= count {
                    break;
                }
            }
        }
        if child == count {
            (arr.wrapping_add(hole.wrapping_mul(SLOT)) as *mut u32)
                .write_unaligned(at(child.wrapping_sub(1)));
            hole = child.wrapping_sub(1);
        }
        lf_checker_rt::callee_cdecl!(SETTLE, u32, arr, hole, start, ctx, extra)
    }
});
